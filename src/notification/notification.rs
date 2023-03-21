use crate::Color;

/// Represents the priority of the notification.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#notificationpriority>.
#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum NotificationPriority {
    PriorityUnspecified,
    PriorityMin,
    PriorityLow,
    PriorityDefault,
    PriorityHigh,
    PriorityMax,
}

/// Represents the visibility of the notification.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#visibility>
#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    VisibilityUnspecified,
    Private,
    Public,
    Secret,
}

/// Represents the notification light settings of the notification.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#lightsettings>.
/// Durations are in seconds.
#[derive(serde::Serialize, Debug, Default, Clone)]
pub struct LightSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    light_on_duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    light_off_duration: Option<String>,
}

impl LightSettings {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = Some(color);
        self
    }

    pub fn light_on_duration(&mut self, seconds: usize) -> &mut Self {
        self.light_on_duration = Some(format!("{}s", seconds));
        self
    }

    pub fn light_off_duration(&mut self, seconds: usize) -> &mut Self {
        self.light_off_duration = Some(format!("{}s", seconds));
        self
    }
}

/// Represents a basic template for notifications, which is equal across all platforms.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#notification>.
/// Use the `Notification::new` function, or initialize the struct yourself:
/// ```rust
/// let notification = Notification {
///     title: Some("Hello, ").to_string(),
///     body: Some("world!").to_string(),
///     image: None,
/// };
/// ```
#[derive(serde::Serialize, Debug, Clone)]
pub struct Notification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}

impl Notification {
    /// Easily create a notification with any type that implements `Into<String>` (such as `&str`).
    /// ```rust
    /// let notification = Notification::new(Some("Hello, "), Some("world!"), None);
    /// ```
    pub fn new(
        title: Option<impl Into<String>>,
        body: Option<impl Into<String>>,
        image: Option<impl Into<String>>,
    ) -> Self {
        Self {
            title: title.map(Into::into),
            body: body.map(Into::into),
            image: image.map(Into::into),
        }
    }
}
