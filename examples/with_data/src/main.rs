use firebae_cm::{
    AndroidConfig, AsFirebaseMap, Client, Message, MessageBody, Notification, Receiver,
};

#[derive(AsFirebaseMap)]
pub struct MessageData {
    pub custom_field: String,
    pub another_type: i32, // Note that this will be converted to a String in firebase
}

#[tokio::main]
async fn main() {
    let data = MessageData {
        custom_field: "Hello, world!".to_string(),
        another_type: 15,
    };

    let notification = Notification::new()
        .with_title("Hello, ")
        .with_body("world!");

    let mut android_config = AndroidConfig::new();
    android_config.direct_boot_ok(true);

    let receiver = Receiver::topic("subscribers");

    // create a message to our receiver(s) and set all configuration
    let mut body = MessageBody::new(receiver);
    body.notification(notification)
        .data(data)
        .expect("Unable to parse data")
        .android(android_config);

    let message = Message::with_oauth("your_project_id", body)
        .await
        .expect("Unable to authenticate");

    // create a client and send the data
    let client = Client::new();
    println!("{:?}", message);
    println!("{:?}", client.send(message).await);
}
