use firebae_cm::{Client, Message, MessageBody, Receiver};

// Make sure to use the GOOGLE_APPLICATION_CREDENTIALS environment variable.
#[tokio::main]
async fn main() {
    let receivers = Receiver::topic("subscribers");
    let body = MessageBody::new(receivers);
    let message = Message::with_oauth("your-project-id", body)
        .await
        .expect("Unable to authenticate");

    let client = Client::new().expect("Unable to build client");
    match client.send(message).await {
        Ok(name) => println!("sent: {name}"),
        Err(e) => eprintln!("failed: {e}"),
    }
}
