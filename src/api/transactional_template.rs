use http_types::{Body, Error as HttpError, Request};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::client::Sendable;

use super::paging::PagingMetadata;

#[derive(Clone, Serialize, Debug)]
pub struct ListTemplatesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generations: Option<GenerationFilter>,
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

impl Sendable for ListTemplatesRequest {
    type Response = ListTemplatesResponse;

    const METHOD: http_types::Method = http_types::Method::Get;

    fn rel_path(&self) -> Cow<'static, str> {
        Cow::Borrowed("templates")
    }

    fn modify_request(&self, req: &mut Request) -> Result<(), HttpError> {
        req.set_query(self)
    }
}

#[derive(Clone, Serialize, Debug)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub generation: Generation,
}

impl Sendable for CreateTemplateRequest {
    type Response = Template;

    const METHOD: http_types::Method = http_types::Method::Post;

    fn rel_path(&self) -> Cow<'static, str> {
        Cow::Borrowed("templates")
    }

    fn modify_request(&self, req: &mut Request) -> Result<(), HttpError> {
        req.set_body(Body::from_json(self)?);
        Ok(())
    }
}
#[derive(Clone, Debug)]
pub struct DeleteTemplateRequest {
    pub template_id: String,
}

impl Sendable for DeleteTemplateRequest {
    type Response = ();

    const METHOD: http_types::Method = http_types::Method::Delete;

    fn rel_path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("templates/{}", self.template_id))
    }
}

#[derive(Clone, Serialize, Debug)]
pub enum GenerationFilter {
    #[serde(rename = "legacy")]
    Legacy,
    #[serde(rename = "dynamic")]
    Dynamic,
    #[serde(rename = "legacy,dynamic")]
    Both
}

#[derive(Clone, Deserialize, Debug)]
pub struct ListTemplatesResponse {
    pub result: Vec<Template>,
    #[serde(rename = "_metadata")]
    pub metadata: PagingMetadata,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Template {
    pub id: String,
    pub name: String,
    pub generation: Generation,
    pub versions: Vec<TemplateVersion>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Generation {
    #[serde(rename = "legacy")]
    Legacy,
    #[serde(rename = "dynamic")]
    Dynamic
}

#[derive(Clone, Deserialize, Debug)]
pub struct TemplateVersion {
    pub id: String,
    pub template_id: String,
    pub name: String,
    pub generate_plain_content: bool,
    pub subject: Option<String>,   
}
