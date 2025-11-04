use serde_json::Value;

use crate::{ApnsFcmOptions, IntoFirebaseMap};

/// Represents all settings for Apple notifications.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#apnsconfig>.
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

    /// Set the headers field. Accepts any type that implements IntoFirebaseMap, which will construct the required Map<String, String>.
    ///
    /// For ease, you can use the [crate::AsFirebaseMap] derive macro on your structs:
    /// ```rust
    /// use firebae_cm::{AsFirebaseMap, ApnsConfig};
    ///
    /// #[derive(AsFirebaseMap)]
    /// struct ApnsHeaders {
    ///     field1: String,
    ///     field2: String,
    /// }
    ///
    /// fn main() {
    ///     let headers = ApnsHeaders {
    ///         field1: "Hello,".to_string(),
    ///         field2: "world!".to_string(),
    ///     };
    ///
    ///     let mut config = ApnsConfig::new();
    ///     config.headers(headers).expect("Data not parsable");
    /// }
    /// ```
    pub fn headers(&mut self, headers: impl IntoFirebaseMap) -> crate::Result<&mut Self> {
        self.headers = Some(serde_json::to_value(headers.as_map().get_map())?);
        Ok(self)
    }

    /// Set the payload field. Accepts any type that implements IntoFirebaseMap, which will construct the required Map<String, String>.
    ///
    /// For ease, you can use the [crate::AsFirebaseMap] derive macro on your structs:
    /// ```rust
    /// use firebae_cm::{AsFirebaseMap, ApnsConfig};
    ///
    /// #[derive(AsFirebaseMap)]
    /// struct ApnsPayload {
    ///     field1: String,
    ///     field2: String,
    /// }
    ///
    /// fn main() {
    ///     let payload = ApnsPayload {
    ///         field1: "Hello,".to_string(),
    ///         field2: "world!".to_string(),
    ///     };
    ///
    ///     let mut config = ApnsConfig::new();
    ///     config.payload(payload).expect("Data not parsable");
    /// }
    /// ```
    pub fn payload(&mut self, payload: Value) -> crate::Result<&mut Self> {
        self.payload = Some(payload);
        Ok(self)
    }

    pub fn fcm_options(&mut self, fcm_options: ApnsFcmOptions) -> &mut Self {
        self.fcm_options = Some(fcm_options);
        self
    }
}
