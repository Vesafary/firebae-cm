//! Tests for the client-side checks that stop a message Firebase would reject.

use firebae_cm::{
    AndroidConfig, Error, FirebaseMap, IntoFirebaseMap, Message, MessageBody, Receiver,
    WebpushConfig,
};

/// A payload whose keys are decided at runtime, so reserved words can be tested
/// without needing a struct field named after each one.
struct RawData(Vec<(&'static str, &'static str)>);

impl IntoFirebaseMap for RawData {
    fn as_map(&self) -> FirebaseMap {
        let mut map = FirebaseMap::new();
        for (key, value) in &self.0 {
            map.insert(*key, value);
        }
        map
    }
}

fn data(key: &'static str) -> RawData {
    RawData(vec![(key, "value")])
}

#[test]
fn reserved_data_keys_are_rejected() {
    for key in [
        "from",
        "message_type",
        "google.",
        "google.anything",
        "gcm.notification.",
        "gcm.notification.title",
    ] {
        let result = MessageBody::new(Receiver::topic("t")).data(data(key));

        match result {
            Err(Error::ReservedDataKey(rejected)) => assert_eq!(rejected, key),
            Err(other) => panic!("wrong error for `{key}`: {other}"),
            Ok(_) => panic!("`{key}` should have been rejected"),
        }
    }
}

#[test]
fn ordinary_data_keys_are_accepted() {
    // `collapse_key` is used by FCM but explicitly permitted in a data payload, and
    // `gcm.` alone is not the reserved prefix — only `gcm.notification.` is.
    for key in [
        "custom_field",
        "collapse_key",
        "googleplex",
        "gcm.other",
        "from_user",
    ] {
        assert!(
            MessageBody::new(Receiver::topic("t"))
                .data(data(key))
                .is_ok(),
            "`{key}` should have been accepted"
        );
    }
}

#[test]
fn platform_specific_data_is_validated_too() {
    assert!(matches!(
        AndroidConfig::new().data(data("from")),
        Err(Error::ReservedDataKey(_))
    ));
    assert!(matches!(
        WebpushConfig::new().data(data("google.x")),
        Err(Error::ReservedDataKey(_))
    ));
}

#[test]
fn headers_are_not_subject_to_the_data_key_rules() {
    // The reserved-word rules apply to `data` only. APNs and webpush headers legally
    // contain keys that would be rejected as data.
    assert!(WebpushConfig::new().headers(data("from")).is_ok());
}

#[test]
fn debug_output_does_not_leak_the_bearer_token() {
    let body = MessageBody::new(Receiver::topic("subscribers"));
    let message = Message::new("my-project", "super-secret-jwt", body);

    let debug = format!("{message:?}");

    assert!(
        !debug.contains("super-secret-jwt"),
        "Debug leaked the JWT: {debug}"
    );
    assert!(
        debug.contains("[redacted]"),
        "expected a redaction marker: {debug}"
    );
    // The non-sensitive fields should still be useful for debugging.
    assert!(debug.contains("my-project"));
    assert!(debug.contains("subscribers"));
}
