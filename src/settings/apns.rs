use serde_json::Value;

use crate::{ApnsFcmOptions, IntoFirebaseMap};

/// Represents all settings for Apple notifications.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#apnsconfig>.
#[derive(serde::Serialize, Debug, Default, Clone)]
#[must_use]
pub struct ApnsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fcm_options: Option<ApnsFcmOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    live_activity_token: Option<String>,
}

impl ApnsConfig {
    /// Creates an empty Apple configuration.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the headers field. Accepts any type that implements IntoFirebaseMap, which will construct the required Map<String, String>.
    ///
    /// For ease, you can use the [crate::AsFirebaseMap] derive macro on your structs:
    /// ```rust
    /// use firebae_cm::{ApnsConfig, AsFirebaseMap};
    ///
    /// #[derive(AsFirebaseMap)]
    /// struct ApnsHeaders {
    ///     field1: String,
    ///     field2: String,
    /// }
    ///
    /// let headers = ApnsHeaders {
    ///     field1: "Hello,".to_string(),
    ///     field2: "world!".to_string(),
    /// };
    ///
    /// let config = ApnsConfig::new().headers(headers).expect("Data not parsable");
    /// ```
    pub fn headers(mut self, headers: impl IntoFirebaseMap) -> crate::Result<Self> {
        self.headers = Some(serde_json::to_value(headers.as_map().into_inner())?);
        Ok(self)
    }

    /// Set the payload field. Unlike most other fields this is a free-form JSON
    /// object rather than a string map, because the APNs payload nests the `aps`
    /// dictionary alongside any custom keys.
    /// See <https://developer.apple.com/documentation/usernotifications/generating-a-remote-notification>.
    /// ```rust
    /// use firebae_cm::ApnsConfig;
    /// use serde_json::json;
    ///
    /// let config = ApnsConfig::new().payload(json!({
    ///     "aps": { "alert": { "title": "Hello, ", "body": "world!" }, "badge": 1 },
    ///     "custom_key": "custom_value",
    /// }));
    /// ```
    pub fn payload(mut self, payload: Value) -> Self {
        self.payload = Some(payload);
        self
    }

    /// Sets the Apple-specific FCM SDK options.
    pub fn fcm_options(mut self, fcm_options: ApnsFcmOptions) -> Self {
        self.fcm_options = Some(fcm_options);
        self
    }

    /// Sets the [Apple Live Activity](https://developer.apple.com/design/human-interface-guidelines/live-activities)
    /// token to send updates to. Accepts either a push token or a push-to-start token.
    pub fn live_activity_token(mut self, live_activity_token: impl Into<String>) -> Self {
        self.live_activity_token = Some(live_activity_token.into());
        self
    }
}
