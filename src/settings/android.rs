use serde_json::Value;

use crate::{
    AndroidNotification,
    AndroidFcmOptions,
};

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AndroidMessagePriority {
    Normal,
    High,
}

#[derive(serde::Serialize, Debug, Default, Clone)]
pub struct AndroidConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    collapse_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<AndroidMessagePriority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restricted_package_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification: Option<AndroidNotification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fcm_options: Option<AndroidFcmOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_boot_ok: Option<bool>,
}

impl AndroidConfig {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn collapse_key(&mut self, collapse_key: String) -> &mut Self {
        self.collapse_key = Some(collapse_key);
        self
    }

    pub fn priority(&mut self, priority: AndroidMessagePriority) -> &mut Self {
        self.priority = Some(priority);
        self
    }

    pub fn ttl(&mut self, seconds: usize) -> &mut Self {
        self.ttl = Some(format!("{}s", seconds));
        self
    }

    pub fn restricted_package_name(&mut self, restricted_package_name: String) -> &mut Self {
        self.restricted_package_name = Some(restricted_package_name);
        self
    }

    pub fn data(&mut self, data: Value) -> &mut Self {
        self.data = Some(data);
        self
    }

    pub fn notification(&mut self, notification: AndroidNotification) -> &mut Self {
        self.notification = Some(notification);
        self
    }

    pub fn fcm_options(&mut self, fcm_options: AndroidFcmOptions) -> &mut Self {
        self.fcm_options = Some(fcm_options);
        self
    }

    pub fn direct_boot_ok(&mut self, direct_boot_ok: bool) -> &mut Self {
        self.direct_boot_ok = Some(direct_boot_ok);
        self
    }
}
