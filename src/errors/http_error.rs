use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpError{
    #[error("Error in the request: {0}")]
    RequestFailed(String),

    #[error("Error in response with code: {0}")]
    ResponseError(u16),

    #[error("Error when deserializing: {0}")]
    DeserializationError(String),
}