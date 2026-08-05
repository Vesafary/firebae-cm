//! Wire-format tests.
//!
//! Every expectation here is taken from the FCM v1 API discovery document
//! (`https://fcm.googleapis.com/$discovery/rest?version=v1`). The enum casing in
//! particular is load-bearing: the protobuf JSON mapping accepts enum *names* or
//! integers only, so a lowercased value is rejected by the API even though the
//! surrounding field names may be written in either snake_case or camelCase.

use firebae_cm::{
    AndroidConfig, AndroidFcmOptions, AndroidMessagePriority, AndroidNotification, ApnsConfig,
    ApnsFcmOptions, Color, FcmOptions, LightSettings, Message, MessageBody, Notification,
    NotificationPriority, Proxy, Receiver, Visibility, WebpushConfig, WebpushFcmOptions,
};
use serde_json::{Value, json};

fn body_json(body: MessageBody) -> Value {
    serde_json::to_value(Message::new("pid", "jwt", body)).unwrap()["message"].clone()
}

#[test]
fn message_wraps_body_and_hides_credentials() {
    let message = Message::new(
        "my-project",
        "secret-jwt",
        MessageBody::new(Receiver::topic("t")),
    );
    let json = serde_json::to_value(&message).unwrap();

    assert_eq!(json, json!({ "message": { "topic": "t" } }));
    // project_id and jwt travel in the URL and the Authorization header, never the body.
    assert!(json.get("project_id").is_none());
    assert!(json.get("jwt").is_none());
}

#[test]
fn validate_only_is_omitted_unless_set() {
    let body = || MessageBody::new(Receiver::topic("t"));

    let default = serde_json::to_value(Message::new("p", "j", body())).unwrap();
    assert!(default.get("validate_only").is_none());

    let dry_run = serde_json::to_value(Message::new("p", "j", body()).validate_only(true)).unwrap();
    assert_eq!(dry_run["validate_only"], json!(true));
}

#[test]
fn receiver_maps_onto_the_target_union() {
    for (receiver, expected) in [
        (Receiver::token("tok"), json!({ "token": "tok" })),
        (Receiver::fid("f1d"), json!({ "fid": "f1d" })),
        (Receiver::topic("news"), json!({ "topic": "news" })),
        (
            Receiver::condition("'a' in topics"),
            json!({ "condition": "'a' in topics" }),
        ),
    ] {
        assert_eq!(body_json(MessageBody::new(receiver)), expected);
    }
}

#[test]
fn empty_body_sends_only_the_target() {
    assert_eq!(
        body_json(MessageBody::new(Receiver::topic("subscribers"))),
        json!({ "topic": "subscribers" })
    );
}

#[test]
fn android_message_priority_uses_protobuf_enum_names() {
    let priority = |p| {
        body_json(MessageBody::new(Receiver::topic("t")).android(AndroidConfig::new().priority(p)))
            ["android"]["priority"]
            .clone()
    };

    assert_eq!(priority(AndroidMessagePriority::Normal), json!("NORMAL"));
    assert_eq!(priority(AndroidMessagePriority::High), json!("HIGH"));
}

#[test]
fn notification_priority_uses_protobuf_enum_names() {
    let priority = |p| {
        let notification = AndroidNotification::new().notification_priority(p);
        body_json(
            MessageBody::new(Receiver::topic("t"))
                .android(AndroidConfig::new().notification(notification)),
        )["android"]["notification"]["notification_priority"]
            .clone()
    };

    assert_eq!(
        priority(NotificationPriority::PriorityUnspecified),
        json!("PRIORITY_UNSPECIFIED")
    );
    assert_eq!(
        priority(NotificationPriority::PriorityMin),
        json!("PRIORITY_MIN")
    );
    assert_eq!(
        priority(NotificationPriority::PriorityLow),
        json!("PRIORITY_LOW")
    );
    assert_eq!(
        priority(NotificationPriority::PriorityDefault),
        json!("PRIORITY_DEFAULT")
    );
    assert_eq!(
        priority(NotificationPriority::PriorityHigh),
        json!("PRIORITY_HIGH")
    );
    assert_eq!(
        priority(NotificationPriority::PriorityMax),
        json!("PRIORITY_MAX")
    );
}

#[test]
fn visibility_uses_protobuf_enum_names() {
    let visibility = |v| {
        let notification = AndroidNotification::new().visibility(v);
        body_json(
            MessageBody::new(Receiver::topic("t"))
                .android(AndroidConfig::new().notification(notification)),
        )["android"]["notification"]["visibility"]
            .clone()
    };

    assert_eq!(
        visibility(Visibility::VisibilityUnspecified),
        json!("VISIBILITY_UNSPECIFIED")
    );
    assert_eq!(visibility(Visibility::Private), json!("PRIVATE"));
    assert_eq!(visibility(Visibility::Public), json!("PUBLIC"));
    assert_eq!(visibility(Visibility::Secret), json!("SECRET"));
}

#[test]
fn proxy_uses_protobuf_enum_names() {
    let proxy = |p| {
        let notification = AndroidNotification::new().proxy(p);
        body_json(
            MessageBody::new(Receiver::topic("t"))
                .android(AndroidConfig::new().notification(notification)),
        )["android"]["notification"]["proxy"]
            .clone()
    };

    assert_eq!(proxy(Proxy::ProxyUnspecified), json!("PROXY_UNSPECIFIED"));
    assert_eq!(proxy(Proxy::Allow), json!("ALLOW"));
    assert_eq!(proxy(Proxy::Deny), json!("DENY"));
    assert_eq!(
        proxy(Proxy::IfPriorityLowered),
        json!("IF_PRIORITY_LOWERED")
    );
}

#[test]
fn durations_are_encoded_with_a_second_suffix() {
    let android = AndroidConfig::new()
        .ttl(3600)
        .notification(AndroidNotification::new().vibrate_timings([0.5, 1.0, 0.5]));
    let json = body_json(MessageBody::new(Receiver::topic("t")).android(android));

    assert_eq!(json["android"]["ttl"], json!("3600s"));
    assert_eq!(
        json["android"]["notification"]["vibrate_timings"],
        json!(["0.5s", "1s", "0.5s"])
    );
}

#[test]
fn light_settings_require_color_and_both_durations() {
    let notification = AndroidNotification::new().light_settings(LightSettings::new(
        Color::rgb(1.0, 0.0, 0.0),
        1.0,
        0.5,
    ));
    let json = body_json(
        MessageBody::new(Receiver::topic("t"))
            .android(AndroidConfig::new().notification(notification)),
    );

    assert_eq!(
        json["android"]["notification"]["light_settings"],
        json!({
            "color": { "red": 1.0, "green": 0.0, "blue": 0.0 },
            "light_on_duration": "1s",
            "light_off_duration": "0.5s",
        })
    );
}

#[test]
fn unset_color_alpha_is_omitted_rather_than_sent_as_zero() {
    // An explicit alpha of 0.0 means fully transparent, whereas an absent alpha means
    // fully opaque. Defaulting the field would silently make LED colors invisible.
    let opaque = serde_json::to_value(Color::rgb(1.0, 0.5, 0.0)).unwrap();
    assert_eq!(opaque, json!({ "red": 1.0, "green": 0.5, "blue": 0.0 }));
    assert!(opaque.get("alpha").is_none());

    let translucent = serde_json::to_value(Color::rgb(1.0, 0.5, 0.0).with_alpha(0.25)).unwrap();
    assert_eq!(translucent["alpha"], json!(0.25));

    assert_eq!(serde_json::to_value(Color::new()).unwrap(), json!({}));
}

#[test]
fn notification_omits_unset_fields() {
    let json = body_json(
        MessageBody::new(Receiver::topic("t")).notification(Notification::new().title("Hello")),
    );

    assert_eq!(json["notification"], json!({ "title": "Hello" }));
}

#[test]
fn apns_payload_is_free_form_json() {
    let apns = ApnsConfig::new()
        .payload(json!({ "aps": { "alert": { "title": "Hi" }, "badge": 3 } }))
        .live_activity_token("live-token");
    let json = body_json(MessageBody::new(Receiver::topic("t")).apns(apns));

    assert_eq!(
        json["apns"],
        json!({
            "payload": { "aps": { "alert": { "title": "Hi" }, "badge": 3 } },
            "live_activity_token": "live-token",
        })
    );
}

#[test]
fn android_config_carries_the_newer_delivery_flags() {
    let android = AndroidConfig::new()
        .direct_boot_ok(true)
        .bandwidth_constrained_ok(true)
        .restricted_satellite_ok(false);
    let json = body_json(MessageBody::new(Receiver::topic("t")).android(android));

    assert_eq!(json["android"]["direct_boot_ok"], json!(true));
    assert_eq!(json["android"]["bandwidth_constrained_ok"], json!(true));
    assert_eq!(json["android"]["restricted_satellite_ok"], json!(false));
}

#[test]
fn fcm_options_are_available_on_every_platform() {
    let json = body_json(
        MessageBody::new(Receiver::topic("t"))
            .fcm_options(FcmOptions::new().analytics_label("all"))
            .android(
                AndroidConfig::new().fcm_options(AndroidFcmOptions::new().analytics_label("droid")),
            )
            .apns(
                ApnsConfig::new().fcm_options(
                    ApnsFcmOptions::new()
                        .analytics_label("apple")
                        .image("https://example.com/a.png"),
                ),
            )
            .webpush(
                WebpushConfig::new().fcm_options(
                    WebpushFcmOptions::new()
                        .analytics_label("web")
                        .link("https://example.com"),
                ),
            ),
    );

    assert_eq!(json["fcm_options"], json!({ "analytics_label": "all" }));
    assert_eq!(
        json["android"]["fcm_options"],
        json!({ "analytics_label": "droid" })
    );
    assert_eq!(
        json["apns"]["fcm_options"],
        json!({ "analytics_label": "apple", "image": "https://example.com/a.png" })
    );
    assert_eq!(
        json["webpush"]["fcm_options"],
        json!({ "analytics_label": "web", "link": "https://example.com" })
    );
}

#[test]
fn webpush_notification_accepts_arbitrary_serializable_options() {
    let webpush = WebpushConfig::new()
        .notification(json!({ "title": "Hi", "icon": "/icon.png" }))
        .unwrap();
    let json = body_json(MessageBody::new(Receiver::topic("t")).webpush(webpush));

    assert_eq!(
        json["webpush"]["notification"],
        json!({ "title": "Hi", "icon": "/icon.png" })
    );
}
