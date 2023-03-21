use std::collections::HashMap;

use serde_json::Value;

use crate::ApnsFcmOptions;

/// Represents all settings for Apple notifications.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#apnsconfig>.
#[derive(serde::Serialize, Debug, Default, Clone)]
pub struct ApnsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fcm_options: Option<ApnsFcmOptions>,
}

impl ApnsConfig {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn headers(&mut self, headers: impl Into<HashMap<String, String>>) -> crate::Result<&mut Self> {
        self.headers = Some(serde_json::to_value(headers.into())?);
        Ok(self)
    }

    pub fn payload(&mut self, payload: impl Into<HashMap<String, String>>) -> crate::Result<&mut Self> {
        self.payload = Some(serde_json::to_value(payload.into())?);
        Ok(self)
    }

    pub fn fcm_options(&mut self, fcm_options: ApnsFcmOptions) -> &mut Self {
        self.fcm_options = Some(fcm_options);
        self
    }
}
