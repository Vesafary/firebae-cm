use time::format_description::well_known::Rfc3339;

use crate::{LightSettings, NotificationPriority, Visibility};

/// Represents the settings for a notification in Android.
/// All settings are optional, and all settings (setter functions) follow the scheme below.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#androidnotification>.
#[derive(serde::Serialize, Debug, Default, Clone)]
pub struct AndroidNotification {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sound: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    click_action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body_loc_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body_loc_args: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title_loc_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title_loc_args: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sticky: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_priority: Option<NotificationPriority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_sound: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_vibrate_timings: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_light_settings: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vibrate_timings: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<Visibility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    light_settings: Option<LightSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<String>,
}

impl AndroidNotification {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn title(&mut self, title: impl Into<String>) -> &mut Self {
        self.title = Some(title.into());
        self
    }

    pub fn body(&mut self, body: impl Into<String>) -> &mut Self {
        self.body = Some(body.into());
        self
    }

    pub fn icon(&mut self, icon: impl Into<String>) -> &mut Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn color(&mut self, color: impl Into<String>) -> &mut Self {
        self.color = Some(color.into());
        self
    }

    pub fn sound(&mut self, sound: impl Into<String>) -> &mut Self {
        self.sound = Some(sound.into());
        self
    }

    pub fn tag(&mut self, tag: impl Into<String>) -> &mut Self {
        self.tag = Some(tag.into());
        self
    }

    pub fn click_action(&mut self, click_action: impl Into<String>) -> &mut Self {
        self.click_action = Some(click_action.into());
        self
    }

    pub fn body_loc_key(&mut self, body_loc_key: impl Into<String>) -> &mut Self {
        self.body_loc_key = Some(body_loc_key.into());
        self
    }

    pub fn body_loc_args(&mut self, body_loc_args: Vec<impl Into<String>>) -> &mut Self {
        self.body_loc_args = Some(body_loc_args.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn title_loc_key(&mut self, title_loc_key: impl Into<String>) -> &mut Self {
        self.title_loc_key = Some(title_loc_key.into());
        self
    }

    pub fn title_loc_args(&mut self, title_loc_args: Vec<impl Into<String>>) -> &mut Self {
        self.title_loc_args = Some(title_loc_args.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn channel_id(&mut self, channel_id: impl Into<String>) -> &mut Self {
        self.channel_id = Some(channel_id.into());
        self
    }

    pub fn ticker(&mut self, ticker: impl Into<String>) -> &mut Self {
        self.ticker = Some(ticker.into());
        self
    }

    pub fn sticky(&mut self, sticky: bool) -> &mut Self {
        self.sticky = Some(sticky);
        self
    }

    pub fn event_time(&mut self, event_time: time::OffsetDateTime) -> crate::Result<&mut Self> {
        self.event_time = Some(event_time.format(&Rfc3339)?);
        Ok(self)
    }

    pub fn local_only(&mut self, local_only: bool) -> &mut Self {
        self.local_only = Some(local_only);
        self
    }

    pub fn notification_priority(
        &mut self,
        notification_priority: NotificationPriority,
    ) -> &mut Self {
        self.notification_priority = Some(notification_priority);
        self
    }

    pub fn default_sound(&mut self, default_sound: bool) -> &mut Self {
        self.default_sound = Some(default_sound);
        self
    }

    pub fn default_vibrate_timings(&mut self, default_vibrate_timings: bool) -> &mut Self {
        self.default_vibrate_timings = Some(default_vibrate_timings);
        self
    }

    pub fn default_light_settings(&mut self, default_light_settings: bool) -> &mut Self {
        self.default_light_settings = Some(default_light_settings);
        self
    }

    pub fn vibrate_timings(&mut self, vibrate_timings: Vec<impl Into<String>>) -> &mut Self {
        self.vibrate_timings = Some(vibrate_timings.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn visibility(&mut self, visibility: Visibility) -> &mut Self {
        self.visibility = Some(visibility);
        self
    }

    pub fn notification_count(&mut self, notification_count: i32) -> &mut Self {
        self.notification_count = Some(notification_count);
        self
    }

    pub fn light_settings(&mut self, light_settings: LightSettings) -> &mut Self {
        self.light_settings = Some(light_settings);
        self
    }

    pub fn image(&mut self, image: impl Into<String>) -> &mut Self {
        self.image = Some(image.into());
        self
    }
}
