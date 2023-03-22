use serde_json::Value;

use crate::{ApnsFcmOptions, IntoFirebaseMap};

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

    /// Set the headers field. Accepts any type that implements IntoFirebaseMap, which will construct the required Map<String, String>.
    pub fn headers(&mut self, headers: impl IntoFirebaseMap) -> crate::Result<&mut Self> {
        self.headers = Some(serde_json::to_value(headers.as_map().get_map())?);
        Ok(self)
    }

    /// Set the payload field. Accepts any type that implements IntoFirebaseMap, which will construct the required Map<String, String>.
    pub fn payload(&mut self, payload: impl IntoFirebaseMap) -> crate::Result<&mut Self> {
        self.payload = Some(serde_json::to_value(payload.as_map().get_map())?);
        Ok(self)
    }

    pub fn fcm_options(&mut self, fcm_options: ApnsFcmOptions) -> &mut Self {
        self.fcm_options = Some(fcm_options);
        self
    }
}
