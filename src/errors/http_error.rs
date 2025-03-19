use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpError{
    #[error("Error in the request: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Error in response with code: {0}")]
    ResponseError(u16),

    #[error("Error when deserializing: {0}")]
    DeserializationError(reqwest::Error),
}