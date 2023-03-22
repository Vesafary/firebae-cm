use gcp_auth::AuthenticationManager;

use firebae_cm::{Client, Error, Message, MessageBody, Receiver};

// Make sure to use the GOOGLE_APPLICATION_CREDENTIALS environment variable.
// (see the gcp_auth crate).
#[tokio::main]
async fn main() {
    let manager = AuthenticationManager::new()
        .await
        .expect("unable to initialize authentication manager");
    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];

    let auth_token = manager
        .get_token(scopes)
        .await
        .expect("Couldn't fetch token");

    let token = auth_token.as_str().trim_end_matches('.');

    let receivers = Receiver::topic("subscribers");
    let body = MessageBody::new(receivers);
    let message = Message::new("your-project-id", token, body);

    let client = Client::new();
    let response: Result<String, Error> = client.send(message).await;
    println!("{:?}", response);
}
