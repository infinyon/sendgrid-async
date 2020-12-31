use async_h1::client;
use async_trait::async_trait;
use http_types::{Error as HttpError, Method as HttpMethod, Request, Response, Url, Body};
use serde::de::DeserializeOwned;
use std::borrow::Cow;
use std::{env, fmt::Debug};
use tracing::*;
use log::Level::Trace;
use log::log_enabled;

use crate::api::ErrorResponse;
use crate::error::SendgridError;

/// Base url for the Sendgrid API.
const DEFAULT_BASE_URL: &str = "https://api.sendgrid.com/v3/";

/// Entrypoint for interacting with the SendGrid API.
#[derive(Clone, Debug)]
pub struct Client {
    base_url: Url,
    key: String,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder {
            base_url: Url::parse(DEFAULT_BASE_URL).expect("error parsing DEFAULT_BASE_URL"),
            key: env::var("SENDGRID_API_KEY").ok(),
        }
    }

    fn create_request(
        &self,
        method: http_types::Method,
        rel_path: &str,
    ) -> Result<Request, HttpError> {
        let url = self.base_url.join(rel_path)?;
        let mut req = Request::new(method, url);
        req.set_content_type("application/json".into());
        self.set_authorization_header(&mut req)?;
        Ok(req)
    }

    fn set_authorization_header(&self, req: &mut Request) -> Result<(), HttpError> {
        use http_types::headers::{HeaderValue, AUTHORIZATION};
        let bt = format!("Bearer {}", self.key);
        let bearer = HeaderValue::from_bytes(bt.into_bytes())?;
        req.append_header(AUTHORIZATION, bearer);
        Ok(())
    }
}

#[derive(Clone)]
pub struct ClientBuilder {
    base_url: Url,
    key: Option<String>,
}

impl ClientBuilder {
    pub fn base_url<S: AsRef<str>>(&mut self, url: S) -> &mut Self {
        self.base_url = Url::parse(url.as_ref()).expect("error parsing DEFAULT_BASE_URL");
        self
    }
    pub fn key<S: Into<String>>(&mut self, key: S) -> &mut Self {
        self.key = Some(key.into());
        self
    }
    pub fn build(&self) -> Result<Client, String> {
        Ok(Client {
            base_url: self.base_url.clone(),
            key: self.key.clone().ok_or("key must be initialized")?,
        })
    }
}

/// A trait for indicating that the implementor can send an API call.
#[async_trait]
pub trait Sendable
where
    Self: Sized,
{
    /// The output object from this API request.
    type Response: Debug + Send + DeserializeOwned + 'static;

    /// HTTP method used by this call.
    const METHOD: HttpMethod;

    /// Relative URL for this API call formatted appropriately with parameter values.
    fn rel_path(&self) -> Cow<'static, str>;

    /// Modify for this request.
    fn modify_request(&self, _req: &mut Request) -> Result<(), HttpError> {
        Ok(())
    }

    /// Convenience method for returning a raw response after sending a request.
    async fn send(
        &self,
        client: &Client,
    ) -> Result<Self::Response, SendgridError<ErrorResponse>> {
        let mut req = client.create_request(Self::METHOD, &self.rel_path())?;

        self.modify_request(&mut req)?;

        trace!("Sending request: {:#?}", req);
        let mut resp = send_http_request(req).await?;
        trace!("Received response: {:#?}", resp);

        if log_enabled!(Trace) {
            let body_string = get_response_body_string(&mut resp).await?;
            trace!("Received response body: {:#?}", body_string);
        }
        
        if resp.status().is_success() {
            // Empty body needs to deserialize to unit struct for mail/send
            if resp.is_empty() == Some(true) {
                resp.set_body("null");
            }
            
            let body = resp.body_json().await?;
            Ok(body)
        } else {
            let body = resp.body_json().await?;
            Err(SendgridError::ApiError(
                resp.status(),
                body,
            ))
        }
    }
}

async fn get_response_body_string(resp: &mut Response) -> Result<String, HttpError> {
    let body = resp.take_body();

    let body_string = body.into_string().await?;
    let body = Body::from_string(body_string.clone());
    resp.set_body(body);
    Ok(body_string)
}

async fn send_http_request(req: Request) -> Result<Response, HttpError> {
    use std::io::{Error as IoError, ErrorKind as IoErrorKind};

    let host = req
        .url()
        .host_str()
        .ok_or_else(|| IoError::new(IoErrorKind::Other, "missing hostname"))?
        .to_string();


        let is_https = req.url().scheme() == "https";

    let addr = req
        .url()
        .socket_addrs(|| if is_https { Some(443) } else { Some(80) })?
        .into_iter()
        .next()
        .ok_or_else(|| IoError::new(IoErrorKind::Other, "could not resolve address"))?;

    let tcp_stream = async_net::TcpStream::connect(addr).await?;

    let result = if is_https {
        let tls_stream = async_native_tls::connect(host, tcp_stream).await?;
        client::connect(tls_stream, req).await
    } else {
        client::connect(tcp_stream, req).await
    };

    result
}
