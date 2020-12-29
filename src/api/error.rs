use serde::Deserialize;
pub use http_types::StatusCode;

#[derive(Debug, Deserialize)]
pub struct ErrorReponse {
    errors: Vec<Error>,
    status_code: StatusCode
}


#[derive(Debug, Deserialize)]
pub struct Error {
    field: Option<String>,
    message: String,
    error_id: Option<String>,
    parameter: Option<String>,
}