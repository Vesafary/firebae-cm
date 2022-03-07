use reqwest::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, RETRY_AFTER};
use reqwest::{Body, StatusCode};

use crate::{Message, FcmResponse, FcmError};

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
            .pool_max_idle_per_host(usize::MAX)
            .build()
            .unwrap();

        Self { client }
    }

    pub async fn send(&self, message: Message) -> crate::Result<String> {
        println!("{}", serde_json::to_string(&message).unwrap());
        let payload = serde_json::to_vec(&message)?;

        let request = self
            .client
            .post(format!("https://fcm.googleapis.com/v1/projects/{}/messages:send", message.project_id))
            .header(CONTENT_TYPE, "application/json; UTF-8")
            .header(CONTENT_LENGTH, format!("{}", payload.len() as u64).as_bytes())
            .header(AUTHORIZATION, format!("Bearer {}", message.jwt).as_bytes())
            .body(Body::from(payload))
            .build()?;

        let response = self.client.execute(request).await?;

        let fcm_response: FcmResponse = response.json().await?;

        match fcm_response {
            FcmResponse::Error(e) => Err(crate::Error::FcmError(e)),
            FcmResponse::Success(s) => Ok(s),
        }
    }
}
