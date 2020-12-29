use async_h1::client;
use async_trait::async_trait;
use http_types::{Error as HttpError, Method as HttpMethod, Request, Response, StatusCode, Url};
use log::debug;
use serde::de::DeserializeOwned;
use std::borrow::Cow;
use std::{env, fmt::Debug};
use tracing::*;

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

/// Common API errors.
#[derive(Debug, thiserror::Error)]
pub enum ClientError<ErrorResponse>
where
    ErrorResponse: Debug + Send + DeserializeOwned + 'static,
{
    #[error("API request error")]
    ApiError(StatusCode, ErrorResponse),
    #[error("An error has occurred while performing the API request: {}", _0)]
    HttpError(HttpError),
}

impl<ErrorResponse> From<HttpError> for ClientError<ErrorResponse>
where
    ErrorResponse: Debug + Send + DeserializeOwned + 'static,
{
    fn from(err: HttpError) -> Self {
        Self::HttpError(err)
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

    type ErrorResponse: Debug + Send + DeserializeOwned + 'static;

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
    ) -> Result<Self::Response, ClientError<Self::ErrorResponse>> {
        let mut req = client.create_request(Self::METHOD, &self.rel_path())?;

        self.modify_request(&mut req)?;

        let mut resp = send_http_request(req).await?;
        trace!("response: {:?}", resp);
        if resp.status().is_success() {
            Ok(resp.body_json().await?)
        } else {
            Err(ClientError::ApiError(
                resp.status(),
                resp.body_json().await?,
            ))
        }
    }
}

async fn send_http_request(req: Request) -> Result<Response, HttpError> {
    use std::io::{Error as IoError, ErrorKind as IoErrorKind};
    debug!("executing request: {:#?}", req);

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

    debug!("http result: {:#?}", result);

    result
}
