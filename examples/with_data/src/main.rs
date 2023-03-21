use std::collections::HashMap;
use firebae_cm::{
    AndroidConfig,
    Client,
    Error,
    Message,
    MessageBody,
    Notification,
    Receiver,
};


pub struct MessageData {
    pub custom_field: String,
}

impl Into<HashMap<String, String>> for MessageData {
    fn into(self) -> HashMap<String, String> {
        let mut h = HashMap::new();
        h.insert("custom_field".to_string(), self.custom_field);
        h
    }
}

#[tokio::main]
async fn main() {
    let data = MessageData {
        custom_field: "Hello, world!".to_string(),
    };

    let notification = Notification::new().with_title("Hello, ").with_body("world!");

    let mut android_config = AndroidConfig::new();
    android_config
        .direct_boot_ok(true);
    
    let receiver = Receiver::topic("subscribers");
    
    // create a message to our receiver(s) and set all configuration
    let mut body = MessageBody::new(receiver);
    body
        .notification(notification)
        .data(data)
        .expect("Unable to parse data")
        .android(android_config);
    
    let message = Message::with_oauth("your_project_id", body).await.expect("Unable to authenticate");
    
    // create a client and send the data
    let client = Client::new();
    println!("{:?}", client.send(message).await);
}