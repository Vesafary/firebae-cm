/// Additional platform independent Firebase options. Contains only the analytics_label.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#fcmoptions>.
#[derive(serde::Serialize, Debug, Clone)]
pub struct FcmOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_label: Option<String>,
}

/// Additional Firebase options for Android. Contains only the analytics_label.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#androidfcmoptions>.
#[derive(serde::Serialize, Debug, Clone)]
pub struct AndroidFcmOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_label: Option<String>,
}

/// Additional Firebase options for Apple.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#apnsfcmoptions>.
#[derive(serde::Serialize, Debug, Clone)]
pub struct ApnsFcmOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}

/// Additional Firebase options for web.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#webpushfcmoptions>.
#[derive(serde::Serialize, Debug, Clone)]
pub struct WebpushFcmOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}
