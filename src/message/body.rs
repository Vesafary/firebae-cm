use serde_json::Value;

use crate::{AndroidConfig, ApnsConfig, Notification, Receiver, WebpushConfig, FcmOptions};


#[derive(serde::Serialize, Debug)]
pub struct MessageBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(flatten)]
    receiver: Receiver,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification: Option<Notification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    android: Option<AndroidConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apns: Option<ApnsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    webpush: Option<WebpushConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fcm_options: Option<FcmOptions>,
}

impl MessageBody {
    pub fn new(receiver: Receiver) -> Self {
        Self {
            name: None,
            receiver,
            data: None,
            notification: None,
            android: None,
            apns: None,
            webpush: None,
            fcm_options: None,
        }
    }

    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn data(&mut self, data: impl serde::Serialize) -> crate::Result<&mut Self> {
        self.data = Some(serde_json::to_value(data)?);
        Ok(self)
    }

    pub fn notification(&mut self, notification: Notification) -> &mut Self {
        self.notification = Some(notification);
        self
    }

    pub fn android(&mut self, android: AndroidConfig) -> &mut Self {
        self.android = Some(android);
        self
    }

    pub fn apns(&mut self, apns: ApnsConfig) -> &mut Self {
        self.apns = Some(apns);
        self
    }

    pub fn webpush(&mut self, webpush: WebpushConfig) -> &mut Self {
        self.webpush = Some(webpush);
        self
    }

    pub fn fcm_options(&mut self, fcm_options: FcmOptions) -> &mut Self {
        self.fcm_options = Some(fcm_options);
        self
    }
}
