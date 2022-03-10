use serde_json::Value;

use crate::{
    ApnsFcmOptions,
};

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

    pub fn headers(&mut self, headers: Value) -> &mut Self {
        self.headers = Some(headers);
        self
    }

    pub fn payload(&mut self, payload: Value) -> &mut Self {
        self.payload = Some(payload);
        self
    }

    pub fn fcm_options(&mut self, fcm_options: ApnsFcmOptions) -> &mut Self {
        self.fcm_options = Some(fcm_options);
        self
    }
}
