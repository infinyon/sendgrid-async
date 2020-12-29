use http_types::{Request, Error as HttpError};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::client::Sendable;

use super::paging::PagingMetadata;

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

impl Sendable for ListTemplatesRequest {
    type Response = ListTemplatesResponse;
    type ErrorResponse = ();

    const METHOD: http_types::Method = http_types::Method::Get;

    fn rel_path(&self) -> Cow<'static, str> {
        Cow::Borrowed("templates")
    }

    fn modify_request(&self, mut req: Request) -> Result<(), HttpError> {
        req.set_query(self)
    }
}

#[derive(Deserialize, Debug)]
pub struct ListTemplatesResponse {
    pub result: Vec<Template>,
    #[serde(rename = "_metadata")]
    pub metadata: PagingMetadata,
}

#[derive(Deserialize, Debug)]
pub struct Template {
    pub id: String,
    pub name: String,
    pub generation: String,
}
