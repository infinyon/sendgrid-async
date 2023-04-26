use http_types::{Request, Result, Error, StatusCode};
use serde::{Deserialize, Serialize};

use super::Client;

#[derive(Deserialize, Debug)]
pub struct TemplateResponse {
    pub id: String,
    pub name: String,
    pub generation: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageToken(String);

#[derive(Deserialize, Debug)]
pub struct Metadata {
    #[serde(rename = "self")]
    pub current: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<PageToken>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<PageToken>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct ListTemplatesResponse {
    pub result: Vec<TemplateResponse>,
    #[serde(rename = "_metadata")]
    pub metadata: Metadata,
}
#[derive(Serialize, Debug)]
pub struct ListTemplatesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generations: Option<String>,
    pub page_size: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl Default for ListTemplatesRequest {
    fn default() -> Self {
        Self {
            generations: None,
            page_size: 200,
            page_token: None,
        }
    }
}

impl ListTemplatesRequest {
    pub async fn execute(&self, client: &Client) -> Result<ListTemplatesResponse> {
        let url = client.url("templates")?;
        let mut request = Request::get(url);
        client.set_authorization_header(&mut request)?;
        request.set_query(self)?;

        let mut response = crate::http_client::execute(request).await?;
        if response.status() != StatusCode::Ok {
            return Err(Error::new(response.status(), anyhow::anyhow!("received unexpected status")))
        }

        response.body_json().await
    }
}
