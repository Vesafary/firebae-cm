use serde_json::Value;

use crate::{
    AndroidFcmOptions, AndroidNotification, IntoFirebaseMap, utils::duration::duration_string,
};

/// Represents the Android message priority.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#androidmessagepriority>.
///
/// Serializes to the protobuf enum names (`NORMAL`, `HIGH`). The FCM JSON parser
/// accepts enum *names* or integers only, so the casing here is significant.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AndroidMessagePriority {
    /// Sent immediately when the device is awake; may be delayed while it is idle.
    Normal,
    /// Wakes a sleeping device to deliver the message. Use only for genuinely time-critical messages.
    High,
}

/// Represents all settings for an Android message.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#AndroidConfig> for a complete list.
///
/// # Example
/// ```rust
/// use firebae_cm::{AndroidConfig, AndroidMessagePriority};
///
/// let config = AndroidConfig::new()
///     .priority(AndroidMessagePriority::High)
///     .ttl(3600)
///     .direct_boot_ok(true);
/// ```
#[derive(serde::Serialize, Debug, Default, Clone)]
#[must_use]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    bandwidth_constrained_ok: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restricted_satellite_ok: Option<bool>,
}

impl AndroidConfig {
    /// Creates an empty Android configuration.
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the group key for collapsible messages: when a device is offline, only the
    /// last message with a given key is delivered once it reconnects.
    pub fn collapse_key(mut self, collapse_key: impl Into<String>) -> Self {
        self.collapse_key = Some(collapse_key.into());
        self
    }

    /// Sets the delivery priority of the message.
    pub fn priority(mut self, priority: AndroidMessagePriority) -> Self {
        self.priority = Some(priority);
        self
    }

    /// Sets how long the message should be kept in FCM storage if the device is
    /// offline, in seconds. The maximum supported time to live is 4 weeks, which is
    /// also the default. Set it to 0 to send the message immediately.
    pub fn ttl(mut self, seconds: usize) -> Self {
        self.ttl = Some(duration_string(seconds as f64));
        self
    }

    /// Restricts delivery to an app with this package name.
    pub fn restricted_package_name(mut self, restricted_package_name: impl Into<String>) -> Self {
        self.restricted_package_name = Some(restricted_package_name.into());
        self
    }

    /// Sets the data of the message. Accepts any type that implements IntoFirebaseMap, which will construct the required Map<String, String>.
    /// For ease, you can use the [crate::AsFirebaseMap] derive macro on your structs:
    /// ```rust
    /// use firebae_cm::{AndroidConfig, AsFirebaseMap};
    ///
    /// #[derive(AsFirebaseMap)]
    /// struct Data {
    ///     field1: String,
    ///     field2: i32, // Note that this will become a String in Firebase
    /// }
    ///
    /// let data = Data {
    ///     field1: "Hello, world!".to_string(),
    ///     field2: 5481,
    /// };
    ///
    /// let config = AndroidConfig::new().data(data).expect("Data not parsable");
    /// ```
    pub fn data(mut self, data: impl IntoFirebaseMap) -> crate::Result<Self> {
        let map = data.as_map();
        map.validate_as_data()?;
        self.data = Some(serde_json::to_value(map.into_inner())?);
        Ok(self)
    }

    /// Sets the Android-specific notification settings.
    pub fn notification(mut self, notification: AndroidNotification) -> Self {
        self.notification = Some(notification);
        self
    }

    /// Sets the Android-specific FCM SDK options.
    pub fn fcm_options(mut self, fcm_options: AndroidFcmOptions) -> Self {
        self.fcm_options = Some(fcm_options);
        self
    }

    /// Allows delivery while the device is in direct boot mode, before the user has unlocked it.
    pub fn direct_boot_ok(mut self, direct_boot_ok: bool) -> Self {
        self.direct_boot_ok = Some(direct_boot_ok);
        self
    }

    /// Allows delivery while the device is in bandwidth constrained mode. Only enable
    /// this when the app has been tested to handle such messages properly.
    pub fn bandwidth_constrained_ok(mut self, bandwidth_constrained_ok: bool) -> Self {
        self.bandwidth_constrained_ok = Some(bandwidth_constrained_ok);
        self
    }

    /// Allows delivery while the device is connected over a restricted satellite
    /// network. Only enable this for messages and apps that can work over such a network.
    pub fn restricted_satellite_ok(mut self, restricted_satellite_ok: bool) -> Self {
        self.restricted_satellite_ok = Some(restricted_satellite_ok);
        self
    }
}
