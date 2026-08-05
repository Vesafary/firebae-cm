use crate::{Color, utils::duration::duration_string};

/// Represents the priority of the notification.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#notificationpriority>.
///
/// Serializes to the protobuf enum names (`PRIORITY_MIN`, `PRIORITY_HIGH`, ...).
/// The FCM JSON parser accepts enum *names* or integers only, so the casing here
/// is significant.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NotificationPriority {
    /// If unspecified, the priority is set to [Self::PriorityDefault].
    PriorityUnspecified,
    /// Lowest priority. May be hidden except in detailed notification logs.
    PriorityMin,
    /// Shown smaller, or lower in the list.
    PriorityLow,
    /// The default priority for notifications that carry no special urgency.
    PriorityDefault,
    /// Shown larger, or higher in the list.
    PriorityHigh,
    /// Highest priority. Used for urgent notifications that need immediate attention.
    PriorityMax,
}

/// Represents the visibility of the notification.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#visibility>
///
/// Serializes to the protobuf enum names (`PRIVATE`, `PUBLIC`, `SECRET`, ...).
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Visibility {
    /// If unspecified, the visibility is set to [Self::Private].
    VisibilityUnspecified,
    /// Shown on all lock screens, but with sensitive content hidden.
    Private,
    /// Shown in its entirety on all lock screens.
    Public,
    /// Not shown at all on a secure lock screen.
    Secret,
}

/// Controls when a notification may be proxied.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#proxy>.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Proxy {
    /// If unspecified, the default proxying behaviour is used.
    ProxyUnspecified,
    /// The notification may be proxied.
    Allow,
    /// The notification may not be proxied.
    Deny,
    /// The notification may be proxied only if its delivery priority was lowered.
    IfPriorityLowered,
}

/// Represents the notification light settings of the notification.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#lightsettings>.
///
/// All three fields are required by Firebase, so they are all taken by the
/// constructor. Durations are in seconds.
/// ```rust
/// use firebae_cm::{Color, LightSettings};
///
/// // A red LED, blinking on for 1s and off for 0.5s.
/// let settings = LightSettings::new(Color::rgb(1.0, 0.0, 0.0), 1.0, 0.5);
/// ```
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[must_use]
pub struct LightSettings {
    color: Color,
    light_on_duration: String,
    light_off_duration: String,
}

impl LightSettings {
    /// Creates the light settings from an LED color and the on/off blink durations
    /// in seconds.
    pub fn new(color: Color, on_seconds: f64, off_seconds: f64) -> Self {
        Self {
            color,
            light_on_duration: duration_string(on_seconds),
            light_off_duration: duration_string(off_seconds),
        }
    }
}

/// Represents a basic template for notifications, which is equal across all platforms.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#notification>.
/// Use the `Notification::new` function, or initialize the struct yourself:
/// ```rust
/// use firebae_cm::Notification;
///
/// let notification = Notification {
///     title: Some("Hello, ".to_string()),
///     body: Some("world!".to_string()),
///     image: None,
/// };
/// ```
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq)]
#[must_use]
pub struct Notification {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The notification's title.
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The notification's body text.
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The URL of an image to display with the notification.
    pub image: Option<String>,
}

impl Notification {
    /// Creates an empty Notification.
    pub fn new() -> Self {
        Default::default()
    }

    /// Easily create a notification with any type that implements `Into<String>` (such as `&str`).
    /// ```rust
    /// use firebae_cm::Notification;
    ///
    /// let notification = Notification::new().title("Hello!");
    /// ```
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Easily create a notification with any type that implements `Into<String>` (such as `&str`).
    /// ```rust
    /// use firebae_cm::Notification;
    ///
    /// let notification = Notification::new().title("Hello, ").body("world!");
    /// ```
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Easily create a notification with any type that implements `Into<String>` (such as `&str`).
    /// ```rust
    /// use firebae_cm::Notification;
    ///
    /// let notification = Notification::new().image("/static/img.png");
    /// ```
    pub fn image(mut self, image: impl Into<String>) -> Self {
        self.image = Some(image.into());
        self
    }
}
