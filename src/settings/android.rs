use serde_json::Value;

use crate::{AndroidFcmOptions, AndroidNotification, IntoFirebaseMap};

/// Represents the Android message priority.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#androidmessagepriority>.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AndroidMessagePriority {
    Normal,
    High,
}

/// Represents all settings for an Android message.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#AndroidConfig> for a complete list.
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

    pub fn collapse_key(&mut self, collapse_key: impl Into<String>) -> &mut Self {
        self.collapse_key = Some(collapse_key.into());
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

    pub fn restricted_package_name(
        &mut self,
        restricted_package_name: impl Into<String>,
    ) -> &mut Self {
        self.restricted_package_name = Some(restricted_package_name.into());
        self
    }

    /// Sets the data of the message. Accepts any type that implements IntoFirebaseMap, which will construct the required Map<String, String>.
    /// For ease, you can use the [crate::AsFirebaseMap] derive macro on your structs:
    /// ```rust
    /// use firebae_cm::{AsFirebaseMap, AndroidConfig};
    ///
    /// #[derive(AsFirebaseMap)]
    /// struct Data {
    ///     field1: String,
    ///     field2: i32, // Note that this will become a String in Firebase
    /// }
    ///
    /// fn main() {
    ///     let data = Data {
    ///         field1: "Hello, world!".to_string(),
    ///         field2: 5481,
    ///     };
    ///
    ///     let mut config = AndroidConfig::new();
    ///     config.data(data).expect("Data not parsable");
    /// }
    /// ```
    pub fn data(&mut self, data: impl IntoFirebaseMap) -> crate::Result<&mut Self> {
        self.data = Some(serde_json::to_value(data.as_map().get_map())?);
        Ok(self)
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
