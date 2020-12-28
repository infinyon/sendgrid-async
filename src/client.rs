use async_h1::client;
use async_trait::async_trait;
use http_types::{Error as HttpError, Method, Request, Response, StatusCode, Url};
use log::debug;
use serde::de::DeserializeOwned;
use std::borrow::Cow;
use std::{env, fmt::Debug};

/// Base url for the Sendgrid API.
const DEFAULT_BASE_URL: &str = "https://api.sendgrid.com/v3/";

/// Entrypoint for interacting with the SendGrid API.
pub struct Client {
    base_url: Url,
    key: String,
}

impl Client {
    /// Create a new SendGrid client struct..
    pub fn new<K>(key: K) -> Self
    where
        K: Into<String>,
    {
        Self {
            base_url: Url::parse(DEFAULT_BASE_URL).expect("error parsing DEFAULT_BASE_URL"),
            key: key.into(),
        }
    }

    /// Create a new SendGrid client struct from environment variables.
    pub fn new_from_env() -> Self {
        let key = env::var("SENDGRID_API_KEY").expect("SENDGRID_API_KEY env variable not set");
        Client::new(key)
    }

    /// Get the currently set API key.
    pub fn key(&self) -> &str {
        &self.key
    }

    fn create_request(
        &self,
        method: http_types::Method,
        rel_path: &str,
    ) -> Result<Request, HttpError> {
        let url = self.base_url.join(rel_path)?;
        let mut req = Request::new(method, url);
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

/// Common API errors.
#[derive(Debug, thiserror::Error)]
pub enum ClientError<ErrorResponse: Debug + Send + DeserializeOwned + 'static> {
    #[error("API request error")]
    ApiError(StatusCode, ErrorResponse),
    // #[error("Unsupported media type in response: {}", _0)]
    // UnsupportedMediaType(http_types::Mime),
    #[error("An error has occurred while performing the API request: {}", _0)]
    HttpError(HttpError),
    // #[error("I/O error: {}", _0)]
    // Io(#[source] std::io::Error),
    // #[error("Error en/decoding \"application/json\" data: {}", _0)]
    // SerdeJson(#[source] serde_json::Error),
}

impl<ErrorResponse: Debug + Send + DeserializeOwned + 'static> From<HttpError>
    for ClientError<ErrorResponse>
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
    const METHOD: Method;

    /// Relative URL for this API call formatted appropriately with parameter values.
    fn rel_path(&self) -> Cow<'static, str>;

    /// Modifier for this object. Builders override this method if they
    /// wish to add query parameters, set body, etc.
    fn modify(&self, req: Request) -> Result<Request, ClientError<Self::ErrorResponse>> {
        Ok(req)
    }

    /// Sends the request and returns a future for the response object.
    // async fn send(
    //     &self,
    //     client: &Client,
    // ) -> Result<Response, ClientError> {
    //     let resp = self.send_raw(client).await?;
    //     let media = resp.media_type();
    //     if let Some(ty) = media {
    //         if media_types::M_0.matches(&ty) {
    //             return ResponseWrapper::wrap(resp, |r| async {
    //                 let bytes = r.body_bytes().await?;
    //                 serde_json::from_reader(bytes.as_ref()).map_err(ApiError::from)
    //             })
    //             .await;
    //         } else if media_types::M_1.matches(&ty) {
    //             return ResponseWrapper::wrap(resp, |r| async {
    //                 let bytes = r.body_bytes().await?;
    //                 serde_yaml::from_reader(bytes.as_ref()).map_err(ApiError::from)
    //             })
    //             .await;
    //         }
    //     }

    //     let ty = resp
    //         .header(http::header::CONTENT_TYPE.as_str())
    //         .map(|v| String::from_utf8_lossy(v.as_bytes()).into_owned())
    //         .unwrap_or_default();
    //     Err(ApiError::UnsupportedMediaType(ty, Mutex::new(resp)))
    // }

    // pub async fn execute(&self, client: &Client) -> Result<ListTemplatesResponse> {
    //     let url = client.url("templates")?;
    //     let mut request = Request::get(url);
    //     client.set_authorization_header(&mut request)?;
    //     request.set_query(self)?;

    //     let mut response = crate::http_client::execute(request).await?;
    //     if response.status() != StatusCode::Ok {
    //         panic!("received response status");
    //     }

    //     Ok(response.body_json().await?)
    // }

    /// Convenience method for returning a raw response after sending a request.
    async fn send(
        &self,
        client: &Client,
    ) -> Result<Self::Response, ClientError<Self::ErrorResponse>> {
        let base_req = client.create_request(Self::METHOD, &self.rel_path())?;
        let req = self.modify(base_req)?;

        let mut resp = send_http_request(req).await?;
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
    debug!("executing request: {:#?}", req);

    let host = req
        .url()
        .host_str()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "missing hostname"))?
        .to_string();

    let scheme = req.url().scheme();
    if scheme != "https" {
        panic!(
            "{} scheme not is supported, only https is supported",
            scheme
        );
    }

    let addr = req
        .url()
        .socket_addrs(|| Some(443))?
        .into_iter()
        .next()
        .ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "could not resolve address")
        })?;

    let raw_stream = async_std::net::TcpStream::connect(addr).await?;

    let stream = async_native_tls::connect(host, raw_stream).await?;

    let result = client::connect(stream, req).await;

    debug!("http result: {:#?}", result);

    result
}
