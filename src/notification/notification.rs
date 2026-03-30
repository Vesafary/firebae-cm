use crate::Color;

/// Represents the priority of the notification.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#notificationpriority>.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
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
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
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
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
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
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct Notification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}

impl Notification {
    /// Creates an empty Notification.
    pub fn new() -> Self {
        Default::default()
    }

    /// Easily create a notification with any type that implements `Into<String>` (such as `&str`).
    /// ```rust
    /// let notification = Notification::new().with_title("Hello!");
    /// ```
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Easily create a notification with any type that implements `Into<String>` (such as `&str`).
    /// ```rust
    /// let notification = Notification::new().with_title("Hello, ").with_body("world!");
    /// ```
    pub fn with_body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Easily create a notification with any type that implements `Into<String>` (such as `&str`).
    /// ```rust
    /// let notification = Notification::new().with_image("/static/img.png");
    /// ```
    pub fn with_image(mut self, image: impl Into<String>) -> Self {
        self.image = Some(image.into());
        self
    }
}
