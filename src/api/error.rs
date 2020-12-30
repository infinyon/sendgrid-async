use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ErrorReponse {
    errors: Vec<Error>,
}


#[derive(Debug, Deserialize)]
pub struct Error {
    field: Option<String>,
    message: String,
    error_id: Option<String>,
    parameter: Option<String>,
}