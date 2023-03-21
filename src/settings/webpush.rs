use std::collections::HashMap;

use serde_json::Value;

use crate::WebpushFcmOptions;

/// Represents all settings for web.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#webpushconfig>.
#[derive(serde::Serialize, Debug, Default, Clone)]
pub struct WebpushConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fcm_options: Option<WebpushFcmOptions>,
}

impl WebpushConfig {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn headers(&mut self, headers: HashMap<String, String>) -> crate::Result<&mut Self> {
        self.headers = Some(serde_json::to_value(headers)?);
        Ok(self)
    }

    pub fn data(&mut self, data: HashMap<String, String>) -> crate::Result<&mut Self> {
        self.data = Some(serde_json::to_value(data)?);
        Ok(self)
    }

    pub fn notification(
        &mut self,
        notification: impl serde::Serialize,
    ) -> crate::Result<&mut Self> {
        self.notification = Some(serde_json::to_value(notification)?);
        Ok(self)
    }

    pub fn fcm_options(&mut self, fcm_options: WebpushFcmOptions) -> &mut Self {
        self.fcm_options = Some(fcm_options);
        self
    }
}
