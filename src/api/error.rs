use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ErrorResponse {
    Single {
        error: String,
    },
    Multiple {
        errors: Option<Vec<Error>>,
    }
}


#[derive(Debug, Deserialize)]
pub struct Error {
    field: Option<String>,
    message: String,
    error_id: Option<String>,
    parameter: Option<String>,
}