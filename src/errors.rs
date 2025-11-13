use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum NaApiError {
    #[error("{0}")]
    IpNotAllowed(String),
    #[error("{0}")]
    APIKeyInvalid(String),
    #[error("{0}")]
    UnknownError(String),
}
