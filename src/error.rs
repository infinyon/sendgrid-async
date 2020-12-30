use serde::de::DeserializeOwned;
use std::fmt::Debug;

/// Common API errors.
#[derive(Debug, thiserror::Error)]
pub enum SendgridError<ErrorResponse>
where
    ErrorResponse: Debug + Send + DeserializeOwned + 'static,
{
    #[error("API request error")]
    ApiError(http_types::StatusCode, ErrorResponse),
    #[error("An error has occurred while performing the API request: {}", _0)]
    HttpError(http_types::Error),
}

impl<ErrorResponse> From<http_types::Error> for SendgridError<ErrorResponse>
where
    ErrorResponse: Debug + Send + DeserializeOwned + 'static,
{
    fn from(err: http_types::Error) -> Self {
        Self::HttpError(err)
    }
}
