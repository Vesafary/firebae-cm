# Changelog

## 0.5.0

This release corrects several places where the serialized message did not match the
FCM v1 API. Verified against the API discovery document
(`https://fcm.googleapis.com/$discovery/rest?version=v1`, revision 20260724).

### Fixed

- **Enum values are now serialized as protobuf enum names.** `AndroidMessagePriority`,
  `NotificationPriority` and `Visibility` previously serialized lowercased
  (`"high"`, `"priority_high"`, `"public"`). The protobuf JSON mapping accepts enum
  *names* or integers only, so Firebase rejected any message that set one of these.
  They now serialize as `HIGH`, `PRIORITY_HIGH`, `PUBLIC` and so on. Field *names*
  were, and remain, fine in snake_case — parsers accept both spellings for those.
- **`Color` components are optional.** They were plain `f32`, so a default `Color`
  serialized `alpha: 0.0` — fully transparent. An omitted alpha means fully opaque,
  which is what an unset value now produces.
- **`MessageBody::name` removed.** `name` is an output-only field: it is the
  identifier Firebase assigns to a sent message and is already returned by
  `Client::send`. It should never have been settable on a request.
- `Client::send` no longer serializes the payload twice, and no longer sets a
  `Content-Length` header that reqwest computes anyway.
- A non-JSON response (an HTML error page from a proxy, say) now produces
  `Error::UnexpectedResponse` with the status and body instead of an opaque decode
  error.

- **`Debug` on `Message` no longer prints the JWT.** The bearer token was excluded
  from the serialized body but not from `Debug`, and the `with_data` example logged
  the whole struct. It now renders as `[redacted]`.
- **The `categories` in `Cargo.toml` were not valid crates.io slugs** (`"Web
  programming"`, `"API bindings"`), which blocks publishing. They are now
  `web-programming` and `api-bindings`.

### Added

- `ErrorCode` and `FcmError::{error_code, raw_error_code, is_retryable}`. Firebase
  reports the actionable failure reason in `error.details[]`, which this crate
  previously discarded — so callers could not distinguish "drop this registration
  token" (`UNREGISTERED`) from "back off and retry" (`QUOTA_EXCEEDED`,
  `UNAVAILABLE`, `INTERNAL`) from "fix the message" (`INVALID_ARGUMENT`). Unknown
  codes are surfaced through `raw_error_code` rather than failing to parse.
- `Client::send_all`, which sends many messages with bounded concurrency. FCM v1 has
  no batch endpoint, so reaching many devices means one request per target.
- Reserved `data` key validation. FCM rejects `from`, `message_type`, and anything
  starting with `google.` or `gcm.notification.`; these now fail locally with
  `Error::ReservedDataKey` instead of costing a round trip. Applies to `data` only,
  not to `headers` or `payload`.
- `#[must_use]` on every builder type. With consuming setters, a statement like
  `body.notification(n);` silently does nothing; this makes it a compile-time warning.
- `Message::validate_only` — Firebase validates the request and reports errors
  without delivering anything.
- `Receiver::Fid` / `Receiver::fid` — Firebase Installation IDs. Firebase has
  deprecated registration tokens in favour of these; `Receiver::token` still works
  and currently also accepts a FID.
- `AndroidConfig::bandwidth_constrained_ok`, `AndroidConfig::restricted_satellite_ok`.
- `AndroidNotification::proxy` (with the new `Proxy` enum) and
  `AndroidNotification::bypass_proxy_notification`.
- `ApnsConfig::live_activity_token`.
- Constructors and chainable setters on `FcmOptions`, `AndroidFcmOptions`,
  `ApnsFcmOptions` and `WebpushFcmOptions`, which previously had only public fields.
- `Client::from_client`, to supply a pre-configured `reqwest::Client`.
- A test suite: wire-format tests for every object, derive macro tests, and response
  parsing tests.

### Changed (breaking)

- **Setters consume and return `Self`.** Every builder is now chainable from its
  constructor, e.g.
  `MessageBody::new(receiver).notification(n).android(cfg)`. Previously most took
  `&mut self` while `Notification` did not.
- **`Notification::with_title` / `with_body` / `with_image` are now `title` / `body` /
  `image`**, matching every other builder.
- **`Client::new` returns `crate::Result<Self>`** instead of panicking on a TLS
  initialization failure. The `Default` impl is gone as a result. The client also
  applies a 30 second request timeout, where previously there was none.
- **`FirebaseMapValue::to_string` is now `to_firebase_value`**, and the trait is
  blanket-implemented for every `Display` type. This removes the shadowing of
  `ToString::to_string`. Implement `Display` for your own map value types.
- **`FirebaseMap::get_map` is now the public `FirebaseMap::into_inner`.**
- **`LightSettings::new` takes its color and both durations**, all three of which
  Firebase requires. The individual setters are gone.
- **`AndroidNotification::vibrate_timings` takes durations in seconds** (`f64`)
  rather than pre-formatted strings, matching `LightSettings` and `ttl`.
- `ApnsConfig::payload` returns `Self` rather than `Result<Self>`; it has no
  fallible path.
- `Error` is `#[non_exhaustive]`, and `crate::Result` is now public. `FcmError` has a
  new `details` field, so struct patterns over it need a `..` arm.
- `Message` and `MessageBody` now implement `Clone`.

### Dependencies

- reqwest 0.11 → 0.13 (the `rustls-tls` feature is now `rustls`)
- gcp_auth 0.7 → 0.12. `AuthenticationManager` was removed upstream; the `oauth`
  feature now uses `gcp_auth::provider()` and the `TokenProvider` trait. The
  undocumented `trim_end_matches('.')` applied to the fetched token has been dropped.
- thiserror 1 → 2, syn 2 → 3 (in firebae-derive), and all other dependencies to
  their current releases.
- Removed the `log` dependency, which was declared but never used.
- Added `futures-util`, for `Client::send_all`.
- Requires firebae-derive 0.2.0.
- Moved to edition 2024, with a declared MSRV of 1.85 checked in CI.
