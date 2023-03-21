/// A Firebase Cloud Message Error. For example, an invalid JWT token would return the following error:
/// ```
/// FcmError { 
///     code: 401, 
///     message: "Request had invalid authentication credentials. Expected OAuth 2 access token, login cookie or other valid authentication credential. See https://developers.google.com/identity/sign-in/web/devconsole-project.", 
///     status: "UNAUTHENTICATED" 
/// }
/// ```
#[derive(serde::Deserialize, thiserror::Error, Debug)]
pub struct FcmError {
    pub code: u16,
    pub message: String,
    pub status: String,
}

impl std::fmt::Display for FcmError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} ({}): {}", self.status, self.code, self.message)
    }
}

/// Represents a response by Firebase, whether it is successful or an error.
#[derive(serde::Deserialize, Debug)]
pub enum FcmResponse {
    #[serde(rename = "name")]
    Success(String),
    #[serde(rename = "error")]
    Error(FcmError),
}
