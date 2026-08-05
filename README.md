# firebae-cm
Rust crate for Firebase Cloud Messaging Http V1

### Information
I recommend to read up on FCM before attempting to use this crate.

| What | Where |
| ---- | ----- |
| Basic information on FCM | https://firebase.google.com/docs/cloud-messaging |
| Used data structure | https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages |


### Usage
``` rust
use firebae_cm::{Client, Error, Message, MessageBody, Receiver};

async fn send_message() -> Result<(), Error> {
    // Setup message
    let receiver = Receiver::topic("subscribers");
    let empty_body = MessageBody::new(receiver);
    let message = Message::new("your_project_id", "your_jwt_token", empty_body);

    // Create client and send message
    let client = Client::new()?;
    let name = client.send(message).await?;
    println!("{name}");
    Ok(())
}
```
This will result in a post request to
`https://fcm.googleapis.com/v1/projects/your_project_id/messages:send`
with the appropriate `Authentication: Bearer your_jwt_token` header
and the following body:
``` json
{
   "message":{
      "topic":"subscribers"
   }
}
```

Setters are chainable and consume `self`, so a fully configured message reads as a
single expression:

``` rust
use firebae_cm::{AndroidConfig, AndroidMessagePriority, MessageBody, Notification, Receiver};

let body = MessageBody::new(Receiver::topic("subscribers"))
    .notification(Notification::new().title("Hello, ").body("world!"))
    .android(AndroidConfig::new().priority(AndroidMessagePriority::High).ttl(3600));
```

### Dry runs
`Message::validate_only(true)` asks Firebase to validate the request and report
errors without delivering anything — useful while shaping a payload.

### Handling failures
`FcmError::error_code` returns the FCM error code, which is what tells you how to
react to a failure:

``` rust
use firebae_cm::{Error, ErrorCode};

fn handle(error: Error) {
    match error {
        Error::FcmError(e) => match e.error_code() {
            // The target is gone; remove it from your database.
            Some(ErrorCode::Unregistered) => {}
            // Transient; resend with exponential backoff.
            Some(code) if code.is_retryable() => {}
            // Permanent; the message itself needs fixing.
            _ => {}
        },
        _ => {}
    }
}
```

### Sending to many devices
FCM v1 has no batch endpoint, so reaching many devices means one request per target.
`Client::send_all(messages, concurrency)` runs them over the shared connection pool
with a bounded number in flight, returning one result per message in input order.

### Receivers
A message targets exactly one of a token, a fid, a topic or a condition. Note that
Firebase has deprecated registration tokens in favour of Firebase Installation IDs
(`Receiver::fid`); `Receiver::token` still works and currently also accepts a FID.

For more useful examples, see the examples folder.
