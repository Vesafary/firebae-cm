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


#[derive(serde::Deserialize, Debug)]
pub enum FcmResponse {
    #[serde(rename = "name")]
    Success(String),
    #[serde(rename = "error")]
    Error(FcmError),
}
