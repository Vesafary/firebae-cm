use serde_json::Value;

use crate::{
    Notification,
    Receiver,
};


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
}

impl MessageBody {
    pub fn new(receiver: Receiver) -> Self {
        Self {
            name: None,
            receiver,
            data: None,
            notification: None,
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
}