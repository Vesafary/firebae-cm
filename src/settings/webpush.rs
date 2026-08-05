use serde_json::Value;

use crate::{IntoFirebaseMap, WebpushFcmOptions};

/// Represents all settings for web.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#webpushconfig>.
#[derive(serde::Serialize, Debug, Default, Clone)]
#[must_use]
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
    /// Creates an empty web push configuration.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the headers field. Accepts any type that implements IntoFirebaseMap, which will construct the required Map<String, String>.
    ///
    /// For ease, you can use the [crate::AsFirebaseMap] derive macro on your structs:
    /// ```rust
    /// use firebae_cm::{AsFirebaseMap, WebpushConfig};
    ///
    /// #[derive(AsFirebaseMap)]
    /// struct WebHeaders {
    ///     field1: String,
    ///     field2: String,
    /// }
    ///
    /// let headers = WebHeaders {
    ///     field1: "Hello,".to_string(),
    ///     field2: "world!".to_string(),
    /// };
    ///
    /// let config = WebpushConfig::new().headers(headers).expect("Data not parsable");
    /// ```
    pub fn headers(mut self, headers: impl IntoFirebaseMap) -> crate::Result<Self> {
        self.headers = Some(serde_json::to_value(headers.as_map().into_inner())?);
        Ok(self)
    }

    /// Set the data field. Accepts any type that implements IntoFirebaseMap, which will construct the required Map<String, String>.
    ///
    /// For ease, you can use the [crate::AsFirebaseMap] derive macro on your structs:
    /// ```rust
    /// use firebae_cm::{AsFirebaseMap, WebpushConfig};
    ///
    /// #[derive(AsFirebaseMap)]
    /// struct WebData {
    ///     field1: String,
    ///     field2: String,
    /// }
    ///
    /// let data = WebData {
    ///     field1: "Hello,".to_string(),
    ///     field2: "world!".to_string(),
    /// };
    ///
    /// let config = WebpushConfig::new().data(data).expect("Data not parsable");
    /// ```
    pub fn data(mut self, data: impl IntoFirebaseMap) -> crate::Result<Self> {
        let map = data.as_map();
        map.validate_as_data()?;
        self.data = Some(serde_json::to_value(map.into_inner())?);
        Ok(self)
    }

    /// Set the notification field. Accepts any type that implements Serialize, matching
    /// the [Web Notification API](https://developer.mozilla.org/en-US/docs/Web/API/Notification)
    /// options.
    pub fn notification(mut self, notification: impl serde::Serialize) -> crate::Result<Self> {
        self.notification = Some(serde_json::to_value(notification)?);
        Ok(self)
    }

    /// Sets the web-specific FCM SDK options.
    pub fn fcm_options(mut self, fcm_options: WebpushFcmOptions) -> Self {
        self.fcm_options = Some(fcm_options);
        self
    }
}
