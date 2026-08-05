/// Additional platform independent Firebase options. Contains only the analytics_label.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#fcmoptions>.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[must_use]
pub struct FcmOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The label associated with the message's analytics data.
    pub analytics_label: Option<String>,
}

impl FcmOptions {
    /// Creates an empty set of options.
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the label associated with the message's analytics data.
    pub fn analytics_label(mut self, analytics_label: impl Into<String>) -> Self {
        self.analytics_label = Some(analytics_label.into());
        self
    }
}

/// Additional Firebase options for Android. Contains only the analytics_label.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#androidfcmoptions>.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[must_use]
pub struct AndroidFcmOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The label associated with the message's analytics data.
    pub analytics_label: Option<String>,
}

impl AndroidFcmOptions {
    /// Creates an empty set of options.
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the label associated with the message's analytics data.
    pub fn analytics_label(mut self, analytics_label: impl Into<String>) -> Self {
        self.analytics_label = Some(analytics_label.into());
        self
    }
}

/// Additional Firebase options for Apple.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#apnsfcmoptions>.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[must_use]
pub struct ApnsFcmOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The label associated with the message's analytics data.
    pub analytics_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The URL of an image to display. Overrides [crate::Notification::image] on Apple devices.
    pub image: Option<String>,
}

impl ApnsFcmOptions {
    /// Creates an empty set of options.
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the label associated with the message's analytics data.
    pub fn analytics_label(mut self, analytics_label: impl Into<String>) -> Self {
        self.analytics_label = Some(analytics_label.into());
        self
    }

    /// Sets the URL of an image to display in the notification. Overrides the
    /// platform independent [crate::Notification::image].
    pub fn image(mut self, image: impl Into<String>) -> Self {
        self.image = Some(image.into());
        self
    }
}

/// Additional Firebase options for web.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#webpushfcmoptions>.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[must_use]
pub struct WebpushFcmOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The label associated with the message's analytics data.
    pub analytics_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The link to open when the user clicks the notification. HTTPS is required.
    pub link: Option<String>,
}

impl WebpushFcmOptions {
    /// Creates an empty set of options.
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the label associated with the message's analytics data.
    pub fn analytics_label(mut self, analytics_label: impl Into<String>) -> Self {
        self.analytics_label = Some(analytics_label.into());
        self
    }

    /// Sets the link to open when the user clicks the notification. HTTPS is required.
    pub fn link(mut self, link: impl Into<String>) -> Self {
        self.link = Some(link.into());
        self
    }
}
