use serde_json::Value;

use crate::{
    WebpushFcmOptions,
};

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

    pub fn headers(&mut self, headers: Value) -> &mut Self {
        self.headers = Some(headers);
        self
    }

    pub fn data(&mut self, data: Value) -> &mut Self {
        self.data = Some(data);
        self
    }

    pub fn notification(&mut self, notification: Value) -> &mut Self {
        self.notification = Some(notification);
        self
    }

    pub fn fcm_options(&mut self, fcm_options: WebpushFcmOptions) -> &mut Self {
        self.fcm_options = Some(fcm_options);
        self
    }
}
