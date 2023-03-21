use reqwest::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE};

use crate::{FcmResponse, Message};

/// A reqwest client wrapper to send Firebase messages.
pub struct Client {
    client: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    /// Creates a client
    pub fn new() -> Self {
        let client = reqwest::ClientBuilder::new().build().unwrap();

        Self { client }
    }

    /// Sends the given message and returns the result.
    /// # Example
    /// ```
    /// use firebae_cm::{Client, Message, MessageBody, Receiver};
    ///
    /// async fn send_message() {
    ///   // Setup message
    ///   let receiver = Receiver::topic("subscribers");
    ///   let empty_body = MessageBody::new(receiver);
    ///   let message = Message::new("your_project_id", "your_jwt_token", empty_body);
    ///
    ///   // Create client and send message
    ///   let client = Client::new();
    ///   let res: Result<String, firebae_cm::Error> = client.send(message).await;
    /// }
    /// ```
    pub async fn send(&self, message: Message) -> crate::Result<String> {
        let payload = serde_json::to_vec(&message)?;

        let fcm_response = self
            .client
            .post(format!(
                "https://fcm.googleapis.com/v1/projects/{}/messages:send",
                message.project_id
            ))
            .header(CONTENT_TYPE, "application/json; UTF-8")
            .header(CONTENT_LENGTH, payload.len())
            .header(AUTHORIZATION, format!("Bearer {}", message.jwt).as_bytes())
            .json(&message)
            .send()
            .await?
            .json()
            .await?;

        match fcm_response {
            FcmResponse::Error(e) => Err(crate::Error::FcmError(e)),
            FcmResponse::Success(s) => Ok(s),
        }
    }
}
