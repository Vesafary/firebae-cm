use firebae_cm::{
    AndroidConfig, AndroidMessagePriority, AsFirebaseMap, Client, Error, ErrorCode, Message,
    MessageBody, Notification, Receiver,
};

#[derive(AsFirebaseMap)]
pub struct MessageData {
    pub custom_field: String,
    pub another_type: i32, // Note that this will be converted to a String in firebase
    pub optional_field: Option<String>, // Omitted entirely when None
}

#[tokio::main]
async fn main() {
    let data = MessageData {
        custom_field: "Hello, world!".to_string(),
        another_type: 15,
        optional_field: None,
    };

    let notification = Notification::new().title("Hello, ").body("world!");

    let android_config = AndroidConfig::new()
        .priority(AndroidMessagePriority::High)
        .direct_boot_ok(true);

    let receiver = Receiver::topic("subscribers");

    // create a message to our receiver(s) and set all configuration
    let body = MessageBody::new(receiver)
        .notification(notification)
        .data(data)
        .expect("Unable to parse data")
        .android(android_config);

    // `validate_only` asks Firebase to check the message without delivering it,
    // which is handy while you are still shaping the payload.
    let message = Message::with_oauth("your_project_id", body)
        .await
        .expect("Unable to authenticate")
        .validate_only(true);

    // create a client and send the data.
    // Debug on Message redacts the JWT, so this is safe to log.
    let client = Client::new().expect("Unable to build client");
    println!("{:?}", message);

    match client.send(message).await {
        Ok(name) => println!("sent: {name}"),
        // The FCM error code is what tells you how to react to a failure.
        Err(Error::FcmError(e)) => match e.error_code() {
            Some(ErrorCode::Unregistered) => {
                println!("target is dead, remove it from the database")
            }
            Some(code) if code.is_retryable() => {
                println!("transient failure ({code:?}), retry with backoff")
            }
            _ => eprintln!("permanent failure: {e}"),
        },
        Err(e) => eprintln!("failed: {e}"),
    }
}
