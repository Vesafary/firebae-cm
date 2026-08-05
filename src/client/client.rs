use std::time::Duration;

use futures_util::{StreamExt, stream};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

use crate::{FcmResponse, Message};

/// Default request timeout. reqwest applies no timeout of its own, which would let a
/// stalled connection hang a caller indefinitely.
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// A reqwest client wrapper to send Firebase messages.
///
/// The client holds a connection pool, so build one and reuse it rather than creating
/// a client per message.
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    /// Creates a client with a 30 second request timeout.
    ///
    /// Fails only if the TLS backend cannot be initialized. Use
    /// [Client::from_client] to supply your own configured [reqwest::Client].
    pub fn new() -> crate::Result<Self> {
        let client = reqwest::ClientBuilder::new()
            .use_rustls_tls()
            .timeout(DEFAULT_TIMEOUT)
            .build()?;

        Ok(Self { client })
    }

    /// Wraps an existing [reqwest::Client], so you can control timeouts, proxies,
    /// connection pooling and so on.
    pub fn from_client(client: reqwest::Client) -> Self {
        Self { client }
    }

    /// Sends the given message and returns the name Firebase assigned to it,
    /// in the format `projects/*/messages/{message_id}`.
    /// # Example
    /// ```no_run
    /// use firebae_cm::{Client, Message, MessageBody, Receiver};
    ///
    /// # async fn run() -> Result<(), firebae_cm::Error> {
    /// // Setup message
    /// let receiver = Receiver::topic("subscribers");
    /// let empty_body = MessageBody::new(receiver);
    /// let message = Message::new("your_project_id", "your_jwt_token", empty_body);
    ///
    /// // Create client and send message
    /// let client = Client::new()?;
    /// let name: String = client.send(message).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(&self, message: Message) -> crate::Result<String> {
        let response = self
            .client
            .post(format!(
                "https://fcm.googleapis.com/v1/projects/{}/messages:send",
                message.project_id
            ))
            .header(CONTENT_TYPE, "application/json; UTF-8")
            .header(AUTHORIZATION, format!("Bearer {}", message.jwt))
            .json(&message)
            .send()
            .await?;

        let status = response.status().as_u16();
        let body = response.text().await?;

        // Parse by hand rather than via `.json()` so that a non-JSON body (an HTML
        // error page from a proxy, say) produces a diagnosable error instead of an
        // opaque decoding failure.
        match serde_json::from_str::<FcmResponse>(&body) {
            Ok(FcmResponse::Error(e)) => Err(crate::Error::FcmError(e)),
            Ok(FcmResponse::Success(name)) => Ok(name),
            Err(_) => Err(crate::Error::UnexpectedResponse { status, body }),
        }
    }

    /// Sends many messages, keeping at most `concurrency` requests in flight.
    ///
    /// The FCM v1 API has no batch endpoint, so reaching many devices means one
    /// request per target. This runs them concurrently over the client's connection
    /// pool instead of serially.
    ///
    /// Results come back in the same order as the input, one per message, and each is
    /// independent — a failure for one target does not affect the others. Inspect
    /// [crate::FcmError::error_code] on the failures to decide which registration
    /// tokens to drop and which to retry.
    ///
    /// `concurrency` is clamped to at least 1. Firebase applies per-project sending
    /// quotas, so pick a value your quota tolerates and back off on
    /// [crate::ErrorCode::QuotaExceeded].
    ///
    /// # Example
    /// ```no_run
    /// use firebae_cm::{Client, Message, MessageBody, Receiver};
    ///
    /// # async fn run() -> Result<(), firebae_cm::Error> {
    /// let messages: Vec<Message> = ["fid-a", "fid-b", "fid-c"]
    ///     .into_iter()
    ///     .map(|fid| {
    ///         let body = MessageBody::new(Receiver::fid(fid));
    ///         Message::new("your_project_id", "your_jwt_token", body)
    ///     })
    ///     .collect();
    ///
    /// let client = Client::new()?;
    /// for result in client.send_all(messages, 16).await {
    ///     match result {
    ///         Ok(name) => println!("sent: {name}"),
    ///         Err(e) => eprintln!("failed: {e}"),
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_all(
        &self,
        messages: impl IntoIterator<Item = Message>,
        concurrency: usize,
    ) -> Vec<crate::Result<String>> {
        stream::iter(messages)
            .map(|message| self.send(message))
            .buffered(concurrency.max(1))
            .collect()
            .await
    }
}
