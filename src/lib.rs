/*!
This crate is a Rust wrapper around the Firebase Cloud Messaging V1 protocol.
This allows you to create cloud messages in pure Rust code and send them via push messages to other devices.

It is highly recommended to read the [Firebase documentation](https://firebase.google.com/docs/cloud-messaging).
In order for this to work, you need a Google Cloud Platform account and Firebase to be setup.

# Usage
This crate can be found [on crates.io](https://crates.io/crates/firebae-cm),
so you can use it by adding `firebae_cm` to your project's `Cargo.toml`.

# Example
```rust
use firebae_cm::{
    Client,
    Error,
    Message,
    MessageBody,
    Receiver,
};

#[tokio::main]
async fn main() {
    let token = "your_jwt_token";

    // Define the receiver mode (Token, Topic or Condition).
    let receivers = Receiver::Topic("subscribers".to_string());

    // Create an empty message to your receiver(s).
    let body = MessageBody::new(receivers);

    // Finalize the message with the correct url and authentication.
    let message = Message::new("project_id", token, body);

    // Create a client and send the data.
    let client = Client::new();
    let response: Result<String, Error> = client.send(message).await;
    println!("{:?}", response);
}
```

# Features
* **oauth** - Enables automatic OAuth authentication.
You will still need Firebase to be setup correctly and the path to a
valid `credentials.json` file in the `GOOGLE_APPLICATION_CREDENTIALS`
environment variable. Then, creating a message can be done using `Message::with_oauth("project_id", body).await?`.
*/

#![cfg_attr(docsrs, feature(doc_cfg))]

mod client;
pub use client::*;

mod message;
pub use message::*;

mod notification;
pub use notification::*;

mod settings;
pub use settings::*;

mod utils;
pub use utils::*;
