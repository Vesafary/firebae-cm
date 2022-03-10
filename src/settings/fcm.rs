#[derive(serde::Serialize, Debug, Clone)]
pub struct FcmOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_label: Option<String>,
}


#[derive(serde::Serialize, Debug, Clone)]
pub struct AndroidFcmOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_label: Option<String>,
}


#[derive(serde::Serialize, Debug, Clone)]
pub struct ApnsFcmOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}


#[derive(serde::Serialize, Debug, Clone)]
pub struct WebpushFcmOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}
