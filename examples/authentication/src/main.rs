use firebae_cm::{Client, Message, MessageBody, Receiver};

// Make sure to use the GOOGLE_APPLICATION_CREDENTIALS environment variable.
// (see the gcp_auth crate).
//
// For most uses the `oauth` feature is simpler; see the managed_oauth example.
#[tokio::main]
async fn main() {
    let provider = gcp_auth::provider()
        .await
        .expect("unable to initialize authentication provider");
    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];

    let auth_token = provider.token(scopes).await.expect("Couldn't fetch token");

    let receivers = Receiver::topic("subscribers");
    let body = MessageBody::new(receivers);
    let message = Message::new("your-project-id", auth_token.as_str(), body);

    let client = Client::new().expect("Unable to build client");
    match client.send(message).await {
        Ok(name) => println!("sent: {name}"),
        Err(e) => eprintln!("failed: {e}"),
    }
}
