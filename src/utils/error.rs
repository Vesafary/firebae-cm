use crate::FcmError;

/// All possible errors that can occur in this crate.
///
/// Marked `#[non_exhaustive]` so that new variants can be added without a breaking
/// release; match with a `_ => ...` arm.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// The HTTP request itself failed, for example on a connection or timeout error.
    #[error("{0}")]
    Request(#[from] reqwest::Error),
    /// A message or payload could not be serialized to JSON.
    #[error("{0}")]
    JsonSerialize(#[from] serde_json::Error),
    /// Firebase rejected the message. Inspect [FcmError::error_code] to decide how to react.
    #[error("{0}")]
    FcmError(#[from] FcmError),
    /// A timestamp could not be formatted as RFC 3339.
    #[error("{0}")]
    TimeFormatError(#[from] time::error::Format),
    /// Firebase returned something that is neither a success nor a structured error,
    /// for example an HTML error page from an intermediate proxy.
    #[error("unexpected response from Firebase (HTTP {status}): {body}")]
    UnexpectedResponse {
        /// The HTTP status of the response.
        status: u16,
        /// The raw response body.
        body: String,
    },
    /// A `data` payload used a key that FCM reserves for itself. Firebase would
    /// reject the message with `INVALID_ARGUMENT`, so it is caught before sending.
    #[error(
        "`{0}` is a reserved FCM data key: keys may not be `from`, `message_type`, \
         or start with `google.` or `gcm.notification.`"
    )]
    ReservedDataKey(String),
}

/// Convenience alias for results returned by this crate.
pub type Result<T> = std::result::Result<T, Error>;
