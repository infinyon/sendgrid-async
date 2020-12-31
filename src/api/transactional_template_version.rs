use http_types::{Body, Error as HttpError, Request};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::client::Sendable;

#[derive(Clone, Debug)]
pub struct RetrieveTemplateVersionRequest {
    pub template_id: String,
    pub version_id: String,
}

impl Sendable for RetrieveTemplateVersionRequest {
    type Response = TemplateVersion;

    const METHOD: http_types::Method = http_types::Method::Get;

    fn rel_path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("templates/{}/versions/{}", self.template_id, self.version_id))
    }    
}

#[derive(Clone, Debug, Serialize)]
pub struct CreateTemplateVersionRequest {
    pub template_id: String,
    pub active: u8,
    pub name: String,
    pub html_content: String,
    pub plain_content: Option<String>,
    pub subject: Option<String>,
    pub test_data: Option<String>,
    pub editor: Option<Editor>,
}

impl Sendable for CreateTemplateVersionRequest {
    type Response = TemplateVersion;

    const METHOD: http_types::Method = http_types::Method::Post;

    fn rel_path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("templates/{}/versions", self.template_id))
    }    

    fn modify_request(&self, req: &mut Request) -> Result<(), HttpError> {
        req.set_body(Body::from_json(self)?);
        Ok(())
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct TemplateVersion {
    pub id: String,
    pub template_id: String,
    pub active: u8,
    pub name: String,
    pub html_content: String,
    pub plain_content: Option<String>,
    pub generate_plain_content: bool,
    pub subject: Option<String>,
    pub test_data: Option<String>,
    pub editor: Editor,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Editor {
    Code,
    Design
}