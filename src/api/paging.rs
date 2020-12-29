use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PageToken(String);

#[derive(Deserialize, Debug)]
pub struct PagingMetadata {
    #[serde(rename = "self")]
    pub current: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<PageToken>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<PageToken>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}