use time::format_description::well_known::Rfc3339;

use crate::{
    LightSettings, NotificationPriority, Proxy, Visibility, utils::duration::duration_string,
};

/// Represents the settings for a notification in Android.
/// All settings are optional, and all settings (setter functions) follow the scheme below.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#androidnotification>.
///
/// # Example
/// ```rust
/// use firebae_cm::{AndroidNotification, NotificationPriority};
///
/// let notification = AndroidNotification::new()
///     .title("Hello, ")
///     .body("world!")
///     .notification_priority(NotificationPriority::PriorityHigh);
/// ```
#[derive(serde::Serialize, Debug, Default, Clone)]
#[must_use]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy: Option<Proxy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bypass_proxy_notification: Option<bool>,
}

impl AndroidNotification {
    /// Creates an empty Android notification.
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the title. Overrides [crate::Notification::title] on Android.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the body. Overrides [crate::Notification::body] on Android.
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Sets the notification icon resource. Defaults to the icon declared in the app manifest.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Sets the notification icon color, in `#rrggbb` format.
    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Sets the sound to play, either `default` or a sound resource bundled in the app.
    pub fn sound(mut self, sound: impl Into<String>) -> Self {
        self.sound = Some(sound.into());
        self
    }

    /// Sets the identifier used to replace an existing notification in the drawer instead of adding a new one.
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }

    /// Sets the activity intent action launched when the user taps the notification.
    pub fn click_action(mut self, click_action: impl Into<String>) -> Self {
        self.click_action = Some(click_action.into());
        self
    }

    /// Sets the key of the body string in the app's string resources, used to localize the body.
    pub fn body_loc_key(mut self, body_loc_key: impl Into<String>) -> Self {
        self.body_loc_key = Some(body_loc_key.into());
        self
    }

    /// Sets the format arguments substituted into the string named by [Self::body_loc_key].
    pub fn body_loc_args(mut self, body_loc_args: Vec<impl Into<String>>) -> Self {
        self.body_loc_args = Some(body_loc_args.into_iter().map(|s| s.into()).collect());
        self
    }

    /// Sets the key of the title string in the app's string resources, used to localize the title.
    pub fn title_loc_key(mut self, title_loc_key: impl Into<String>) -> Self {
        self.title_loc_key = Some(title_loc_key.into());
        self
    }

    /// Sets the format arguments substituted into the string named by [Self::title_loc_key].
    pub fn title_loc_args(mut self, title_loc_args: Vec<impl Into<String>>) -> Self {
        self.title_loc_args = Some(title_loc_args.into_iter().map(|s| s.into()).collect());
        self
    }

    /// Sets the notification channel id (Android O and above). The app must have created the channel; otherwise FCM uses the default channel.
    pub fn channel_id(mut self, channel_id: impl Into<String>) -> Self {
        self.channel_id = Some(channel_id.into());
        self
    }

    /// Sets the text sent to accessibility services when the notification arrives.
    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
        self
    }

    /// When true, the notification stays in the drawer after the user taps it.
    pub fn sticky(mut self, sticky: bool) -> Self {
        self.sticky = Some(sticky);
        self
    }

    /// Sets the time the event in the notification occurred, so that notifications can be
    /// sorted by it. Encoded as RFC 3339.
    pub fn event_time(mut self, event_time: time::OffsetDateTime) -> crate::Result<Self> {
        self.event_time = Some(event_time.format(&Rfc3339)?);
        Ok(self)
    }

    /// When true, the notification is not bridged to other devices such as a paired watch.
    pub fn local_only(mut self, local_only: bool) -> Self {
        self.local_only = Some(local_only);
        self
    }

    /// Sets how the notification is presented to the user. This is distinct from
    /// [crate::AndroidMessagePriority], which controls delivery rather than display.
    pub fn notification_priority(mut self, notification_priority: NotificationPriority) -> Self {
        self.notification_priority = Some(notification_priority);
        self
    }

    /// Uses the app's default notification sound.
    pub fn default_sound(mut self, default_sound: bool) -> Self {
        self.default_sound = Some(default_sound);
        self
    }

    /// Uses the app's default vibration pattern, overriding [Self::vibrate_timings] if both are set.
    pub fn default_vibrate_timings(mut self, default_vibrate_timings: bool) -> Self {
        self.default_vibrate_timings = Some(default_vibrate_timings);
        self
    }

    /// Uses the app's default LED settings, overriding [Self::light_settings] if both are set.
    pub fn default_light_settings(mut self, default_light_settings: bool) -> Self {
        self.default_light_settings = Some(default_light_settings);
        self
    }

    /// Sets the vibration pattern, as durations in seconds. The first value is how
    /// long to wait before turning the vibrator on, the next how long to keep it on,
    /// and subsequent values alternate off and on.
    /// ```rust
    /// use firebae_cm::AndroidNotification;
    ///
    /// // Wait 0.5s, vibrate for 1s, pause 0.5s, vibrate for 1s.
    /// let notification = AndroidNotification::new().vibrate_timings([0.5, 1.0, 0.5, 1.0]);
    /// ```
    pub fn vibrate_timings(mut self, seconds: impl IntoIterator<Item = f64>) -> Self {
        self.vibrate_timings = Some(seconds.into_iter().map(duration_string).collect());
        self
    }

    /// Sets how much of the notification is shown on a secure lock screen.
    pub fn visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = Some(visibility);
        self
    }

    /// Sets how many items this notification represents, shown as a badge count on the launcher icon.
    pub fn notification_count(mut self, notification_count: i32) -> Self {
        self.notification_count = Some(notification_count);
        self
    }

    /// Sets the LED color and blink rate.
    pub fn light_settings(mut self, light_settings: LightSettings) -> Self {
        self.light_settings = Some(light_settings);
        self
    }

    /// Sets the URL of an image to display. Overrides [crate::Notification::image] on Android.
    pub fn image(mut self, image: impl Into<String>) -> Self {
        self.image = Some(image.into());
        self
    }

    /// Controls when this notification may be proxied.
    pub fn proxy(mut self, proxy: Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }

    /// If set, display notifications delivered to the device are handled by the app
    /// instead of the proxy.
    pub fn bypass_proxy_notification(mut self, bypass_proxy_notification: bool) -> Self {
        self.bypass_proxy_notification = Some(bypass_proxy_notification);
        self
    }
}
