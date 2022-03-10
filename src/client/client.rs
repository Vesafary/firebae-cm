use reqwest::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE};

use crate::{Message, FcmResponse};

pub struct Client {
    client: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    pub fn new() -> Self {
        let client = reqwest::ClientBuilder::new()
            .build()
            .unwrap();

        Self { client }
    }

    pub async fn send(&self, message: Message) -> crate::Result<String> {
        let payload = serde_json::to_vec(&message)?;

        let fcm_response = self
            .client
            .post(format!("https://fcm.googleapis.com/v1/projects/{}/messages:send", message.project_id))
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
