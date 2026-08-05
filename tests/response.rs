//! Tests for parsing the two shapes Firebase can return from `messages:send`.

use firebae_cm::{ErrorCode, FcmError, FcmResponse};

#[test]
fn success_carries_the_assigned_message_name() {
    let json = r#"{"name":"projects/my-project/messages/0:1500415314455276%31bd1c9631bd1c96"}"#;

    match serde_json::from_str::<FcmResponse>(json).unwrap() {
        FcmResponse::Success(name) => {
            assert_eq!(
                name,
                "projects/my-project/messages/0:1500415314455276%31bd1c9631bd1c96"
            )
        }
        other => panic!("expected success, got {other:?}"),
    }
}

#[test]
fn error_carries_code_status_and_message() {
    let json = r#"{"error":{"code":401,"message":"Request had invalid authentication credentials.","status":"UNAUTHENTICATED"}}"#;

    match serde_json::from_str::<FcmResponse>(json).unwrap() {
        FcmResponse::Error(FcmError {
            code,
            message,
            status,
            ..
        }) => {
            assert_eq!(code, 401);
            assert_eq!(status, "UNAUTHENTICATED");
            assert_eq!(message, "Request had invalid authentication credentials.");
        }
        other => panic!("expected error, got {other:?}"),
    }
}

#[test]
fn error_display_includes_all_three_parts() {
    let error = FcmError {
        code: 404,
        message: "Requested entity was not found.".to_string(),
        status: "NOT_FOUND".to_string(),
        details: Vec::new(),
    };

    assert_eq!(
        error.to_string(),
        "NOT_FOUND (404): Requested entity was not found."
    );
}

#[test]
fn a_non_fcm_body_fails_to_parse() {
    // The client turns this case into Error::UnexpectedResponse rather than an
    // opaque decode failure.
    assert!(serde_json::from_str::<FcmResponse>("<html>502 Bad Gateway</html>").is_err());
}

/// Builds the error shape Firebase actually sends, with the FCM-specific entry in
/// `details` alongside an unrelated extension.
fn error_with_code(http_code: u16, status: &str, error_code: &str) -> FcmError {
    let json = format!(
        r#"{{"error":{{"code":{http_code},"message":"boom","status":"{status}","details":[
             {{"@type":"type.googleapis.com/google.rpc.BadRequest","fieldViolations":[]}},
             {{"@type":"type.googleapis.com/google.firebase.fcm.v1.FcmError","errorCode":"{error_code}"}}
           ]}}}}"#
    );

    match serde_json::from_str::<FcmResponse>(&json).unwrap() {
        FcmResponse::Error(e) => e,
        other => panic!("expected error, got {other:?}"),
    }
}

#[test]
fn every_documented_error_code_is_recognized() {
    for (wire, expected) in [
        ("UNSPECIFIED_ERROR", ErrorCode::UnspecifiedError),
        ("INVALID_ARGUMENT", ErrorCode::InvalidArgument),
        ("UNREGISTERED", ErrorCode::Unregistered),
        ("SENDER_ID_MISMATCH", ErrorCode::SenderIdMismatch),
        ("QUOTA_EXCEEDED", ErrorCode::QuotaExceeded),
        ("UNAVAILABLE", ErrorCode::Unavailable),
        ("INTERNAL", ErrorCode::Internal),
        ("THIRD_PARTY_AUTH_ERROR", ErrorCode::ThirdPartyAuthError),
    ] {
        let error = error_with_code(400, "INVALID_ARGUMENT", wire);
        assert_eq!(error.error_code(), Some(expected), "for {wire}");
        assert_eq!(error.raw_error_code(), Some(wire));
    }
}

#[test]
fn the_fcm_detail_is_picked_out_from_among_other_extensions() {
    // google.rpc.BadRequest is listed first; the FCM entry must still be found.
    let error = error_with_code(404, "NOT_FOUND", "UNREGISTERED");
    assert_eq!(error.details.len(), 2);
    assert_eq!(error.error_code(), Some(ErrorCode::Unregistered));
}

#[test]
fn an_unknown_error_code_is_surfaced_raw_rather_than_breaking_parsing() {
    let error = error_with_code(400, "INVALID_ARGUMENT", "SOME_FUTURE_CODE");

    assert_eq!(error.error_code(), None);
    assert_eq!(error.raw_error_code(), Some("SOME_FUTURE_CODE"));
}

#[test]
fn retryability_follows_the_error_code() {
    let retryable = ["QUOTA_EXCEEDED", "UNAVAILABLE", "INTERNAL"];
    let permanent = [
        "UNSPECIFIED_ERROR",
        "INVALID_ARGUMENT",
        "UNREGISTERED",
        "SENDER_ID_MISMATCH",
        "THIRD_PARTY_AUTH_ERROR",
    ];

    for wire in retryable {
        assert!(
            error_with_code(429, "RESOURCE_EXHAUSTED", wire).is_retryable(),
            "{wire}"
        );
    }
    for wire in permanent {
        assert!(
            !error_with_code(400, "INVALID_ARGUMENT", wire).is_retryable(),
            "{wire}"
        );
    }
}

#[test]
fn retryability_falls_back_to_the_http_status_without_a_code() {
    let bare = |code| FcmError {
        code,
        message: "boom".to_string(),
        status: "UNKNOWN".to_string(),
        details: Vec::new(),
    };

    assert!(bare(429).is_retryable());
    assert!(bare(500).is_retryable());
    assert!(bare(503).is_retryable());
    assert!(!bare(400).is_retryable());
    assert!(!bare(401).is_retryable());
    assert!(!bare(404).is_retryable());
}

#[test]
fn display_includes_the_error_code_when_present() {
    let error = error_with_code(404, "NOT_FOUND", "UNREGISTERED");
    assert_eq!(error.to_string(), "NOT_FOUND (404, UNREGISTERED): boom");
}

#[test]
fn details_default_to_empty_when_firebase_omits_them() {
    let json = r#"{"error":{"code":401,"message":"nope","status":"UNAUTHENTICATED"}}"#;

    match serde_json::from_str::<FcmResponse>(json).unwrap() {
        FcmResponse::Error(e) => {
            assert!(e.details.is_empty());
            assert_eq!(e.error_code(), None);
            assert_eq!(e.raw_error_code(), None);
        }
        other => panic!("expected error, got {other:?}"),
    }
}
