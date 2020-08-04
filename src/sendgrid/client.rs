use http_types::{Body, Error, Request, Result, StatusCode, Url};
use std::env;

use super::*;

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

    pub fn url(&self, path: &str) -> Result<Url> {
        Ok(self.base_url.join(path)?)
    }

    /// Send a sendgrid message.
    pub async fn send_message(&self, message: &Message) -> Result<()> {
        let url = self.url("mail/send")?;
        let mut request = Request::post(url);
        self.set_authorization_header(&mut request)?;
        request.set_body(Body::from_json(message)?);

        let mut resp = crate::http_client::execute(request).await?;
        match resp.status() {
            StatusCode::Accepted => Ok(()),
            s => {
                let body = resp.body_string().await?;
                let error = anyhow::Error::msg(body);
                Err(Error::new(s, error))
            }
        }
    }

    pub fn set_authorization_header(&self, req: &mut Request) -> Result<()> {
        use http_types::headers::*;
        let bt = format!("Bearer {}", self.key);
        let bearer = HeaderValue::from_bytes(bt.into_bytes())?;
        req.append_header(AUTHORIZATION, bearer);
        Ok(())
    }
}
