use serde_json::Value;

use crate::{AndroidConfig, ApnsConfig, Notification, Receiver};


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
}