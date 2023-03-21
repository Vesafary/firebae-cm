use std::collections::HashMap;

use serde_json::Value;

use crate::{AndroidConfig, ApnsConfig, FcmOptions, Notification, Receiver, WebpushConfig};

/// A message body as described in <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#resource:-message>.
///
/// For initialization, it requires a [Receiver](Receiver), which is either a token, a topic or a condition.
///
/// After initialization, fields can be set using the setter functions.
///
/// # Example
/// ```rust
/// use firebae_cm::{MessageBody, Notification, Receiver};
///
/// // Setup of message fields.
/// let receiver = Receiver::topic("subscribers");
/// let notification = Notification::new(Some("Hello, "), Some("world!"), None);
///
/// // Create MessageBody and set the message name and notification.
/// let mut body = MesageBody::new(receiver);
/// body.name("Celebration")
///     .notification(notification);
/// ```
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
    /// Creates a new MessageBody using the supplied Receiver.
    /// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#resource:-message>.
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

    /// Sets the name of the message.
    /// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#resource:-message>.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    /// Sets the data of the message.
    /// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#resource:-message>.
    pub fn data(&mut self, data: impl Into<HashMap<String, String>>) -> crate::Result<&mut Self> {
        self.data = Some(serde_json::to_value(data.into())?);
        Ok(self)
    }

    /// Sets the notification of the message.
    /// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#notification>.
    pub fn notification(&mut self, notification: Notification) -> &mut Self {
        self.notification = Some(notification);
        self
    }

    /// Sets the Android configuration for the message.
    /// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#androidconfig>.
    pub fn android(&mut self, android: AndroidConfig) -> &mut Self {
        self.android = Some(android);
        self
    }

    /// Sets the Apple configuration for the message.
    /// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#apnsconfig>.
    pub fn apns(&mut self, apns: ApnsConfig) -> &mut Self {
        self.apns = Some(apns);
        self
    }

    /// Sets the web configuration for the message.
    /// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#webpushconfig>.
    pub fn webpush(&mut self, webpush: WebpushConfig) -> &mut Self {
        self.webpush = Some(webpush);
        self
    }

    /// Sets additional Firebase options for the message.
    /// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#fcmoptions>.
    pub fn fcm_options(&mut self, fcm_options: FcmOptions) -> &mut Self {
        self.fcm_options = Some(fcm_options);
        self
    }
}
