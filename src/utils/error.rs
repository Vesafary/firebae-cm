use crate::FcmError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Request(#[from] reqwest::Error),
    #[error("{0}")]
    JsonSerialize(#[from] serde_json::Error),
    #[error("{0}")]
    Other(String, u16),
    #[error("{0}")]
    FcmError(#[from] FcmError),
    #[error("{0}")]
    TimeFormatError(#[from] time::error::Format),
}

pub type Result<T> = std::result::Result<T, Error>;
