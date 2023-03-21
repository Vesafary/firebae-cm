use firebae_cm::{
    Client,
    Error,
    Message,
    MessageBody,
    Receiver,
};


// Make sure to use the GOOGLE_APPLICATION_CREDENTIALS environment variable.
#[tokio::main]
async fn main() {
    let receivers = Receiver::Topic("subscribers".to_string());
    let body = MessageBody::new(receivers);
    let message = Message::with_oauth("your-project-id", body).await.expect("Unable to authenticate");

    let client = Client::new();
    let response: Result<String, Error> = client.send(message).await;
    println!("{:?}", response);
}
