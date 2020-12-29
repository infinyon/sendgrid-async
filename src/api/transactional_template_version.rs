
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::client::Sendable;

#[derive(Debug)]
pub struct RetrieveTemplateVersionRequest {
    pub template_id: String,
    pub version_id: String,
}

impl Sendable for RetrieveTemplateVersionRequest {
    type Response = TemplateVersion;
    type ErrorResponse = super::ErrorReponse;

    const METHOD: http_types::Method = http_types::Method::Get;

    fn rel_path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("templates/{}/versions/{}", self.template_id, self.version_id))
    }    
}

#[derive(Debug, Serialize)]
pub struct CreateTemplateVersionRequest {
    pub template_id: String,
}

impl Sendable for CreateTemplateVersionRequest {
    type Response = TemplateVersion;
    type ErrorResponse = super::ErrorReponse;

    const METHOD: http_types::Method = http_types::Method::Get;

    fn rel_path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("templates/{}/versions", self.template_id))
    }    
}

#[derive(Deserialize, Debug)]
pub struct TemplateVersion {
    id: String,
    template_id: String,
    name: String,
    html_content: String,
    plain_content: Option<String>,
    generate_plain_content: bool,
    subject: String,   
}
