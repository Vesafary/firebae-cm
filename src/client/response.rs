use serde_json::Value;

/// The `@type` marker Firebase uses for the FCM-specific entry in an error's
/// `details` array.
const FCM_ERROR_DETAIL_TYPE: &str = "type.googleapis.com/google.firebase.fcm.v1.FcmError";

/// Error codes for FCM failure conditions.
///
/// This is the actionable part of a failure: it distinguishes "drop this token"
/// from "retry later" from "your message is malformed", which the HTTP status alone
/// does not.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/ErrorCode>.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorCode {
    /// No more information is available about this error.
    UnspecifiedError,
    /// HTTP 400. Request parameters were invalid: bad registration token, package
    /// name mismatch, oversized message, reserved data key, or an out-of-range TTL.
    /// Retrying will not help.
    InvalidArgument,
    /// HTTP 404. The app instance was unregistered from FCM. The registration token
    /// is dead and should be removed from your database.
    Unregistered,
    /// HTTP 403. The authenticated sender ID differs from the sender ID for the
    /// registration token.
    SenderIdMismatch,
    /// HTTP 429. Sending limit exceeded for the message target. Retry with
    /// exponential backoff.
    QuotaExceeded,
    /// HTTP 503. The server is overloaded. Retry with exponential backoff.
    Unavailable,
    /// HTTP 500. An unknown internal error occurred. Retry with exponential backoff.
    Internal,
    /// HTTP 401. The APNs certificate or web push auth key was invalid or missing.
    ThirdPartyAuthError,
}

impl ErrorCode {
    /// Parses the string Firebase sends in the `errorCode` field. Returns `None` for
    /// values this crate does not know about, so that a code added by Firebase in the
    /// future does not break response parsing.
    fn from_wire(value: &str) -> Option<Self> {
        Some(match value {
            "UNSPECIFIED_ERROR" => Self::UnspecifiedError,
            "INVALID_ARGUMENT" => Self::InvalidArgument,
            "UNREGISTERED" => Self::Unregistered,
            "SENDER_ID_MISMATCH" => Self::SenderIdMismatch,
            "QUOTA_EXCEEDED" => Self::QuotaExceeded,
            "UNAVAILABLE" => Self::Unavailable,
            "INTERNAL" => Self::Internal,
            "THIRD_PARTY_AUTH_ERROR" => Self::ThirdPartyAuthError,
            _ => return None,
        })
    }

    /// Whether resending the same message could plausibly succeed.
    ///
    /// True for [ErrorCode::QuotaExceeded], [ErrorCode::Unavailable] and
    /// [ErrorCode::Internal]. Firebase asks that retries use exponential backoff.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::QuotaExceeded | Self::Unavailable | Self::Internal
        )
    }
}

/// A Firebase Cloud Message Error. For example, an invalid JWT token would return the following error:
/// ```text
/// FcmError {
///     code: 401,
///     message: "Request had invalid authentication credentials. Expected OAuth 2 access token, login cookie or other valid authentication credential. See https://developers.google.com/identity/sign-in/web/devconsole-project.",
///     status: "UNAUTHENTICATED"
/// }
/// ```
#[derive(serde::Deserialize, thiserror::Error, Debug, Clone)]
pub struct FcmError {
    /// The HTTP status code of the failure.
    pub code: u16,
    /// A human readable description of the failure.
    pub message: String,
    /// The canonical google.rpc status, e.g. `UNAUTHENTICATED` or `NOT_FOUND`.
    pub status: String,
    /// Additional error details. Firebase puts the FCM-specific error code here,
    /// alongside other extensions such as `google.rpc.BadRequest`. Prefer the
    /// [FcmError::error_code] accessor over reading this directly.
    #[serde(default)]
    pub details: Vec<Value>,
}

impl FcmError {
    /// The raw `errorCode` string from the FCM-specific entry of `details`, if present.
    ///
    /// Useful for logging a code that [FcmError::error_code] does not recognize.
    pub fn raw_error_code(&self) -> Option<&str> {
        self.details
            .iter()
            .find(|detail| {
                detail.get("@type").and_then(Value::as_str) == Some(FCM_ERROR_DETAIL_TYPE)
            })
            .and_then(|detail| detail.get("errorCode"))
            .and_then(Value::as_str)
    }

    /// The FCM error code, if Firebase supplied one this crate recognizes.
    ///
    /// This is what tells you whether to drop a registration token
    /// ([ErrorCode::Unregistered]), back off ([ErrorCode::QuotaExceeded]) or fix the
    /// message ([ErrorCode::InvalidArgument]).
    pub fn error_code(&self) -> Option<ErrorCode> {
        self.raw_error_code().and_then(ErrorCode::from_wire)
    }

    /// Whether resending the same message could plausibly succeed.
    ///
    /// Uses the FCM error code when one is present, and otherwise falls back to the
    /// HTTP status: 429, 500 and 503 are treated as retryable.
    pub fn is_retryable(&self) -> bool {
        match self.error_code() {
            Some(code) => code.is_retryable(),
            None => matches!(self.code, 429 | 500 | 503),
        }
    }
}

impl std::fmt::Display for FcmError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.raw_error_code() {
            Some(error_code) => write!(
                f,
                "{} ({}, {}): {}",
                self.status, self.code, error_code, self.message
            ),
            None => write!(f, "{} ({}): {}", self.status, self.code, self.message),
        }
    }
}

/// Represents a response by Firebase, whether it is successful or an error.
#[derive(serde::Deserialize, Debug)]
pub enum FcmResponse {
    /// The name Firebase assigned to the sent message, in the format
    /// `projects/*/messages/{message_id}`.
    #[serde(rename = "name")]
    Success(String),
    /// The message was rejected.
    #[serde(rename = "error")]
    Error(FcmError),
}
