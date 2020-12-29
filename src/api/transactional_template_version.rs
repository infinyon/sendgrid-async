
use http_types::{Body, Request, Error as HttpError};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::client::Sendable;

use super::paging::PagingMetadata;

#[derive(Debug)]
pub struct RetrieveTemplateVersionRequest {
    pub template_id: String,
    pub version_id: String,
}

impl Sendable for RetrieveTemplateVersionRequest {
    type Response = RetrieveTemplateVersionResponse;
    type ErrorResponse = super::ErrorReponse;

    const METHOD: http_types::Method = http_types::Method::Get;

    fn rel_path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("templates/{}/versions/{}", self.template_id, self.version_id))
    }    
}

#[derive(Deserialize, Debug)]
pub struct RetrieveTemplateVersionResponse {
    id: String,
    template_id: String,
    name: String,
    html_content: String,
    plain_content: Option<String>,
    generate_plain_content: bool,
    subject: String,   
}
