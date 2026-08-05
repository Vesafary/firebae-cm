use crate::MessageBody;

/// A representation of a complete message that can be sent. It requires your project_id and an authentication JWT token (see <https://cloud.google.com/docs/authentication/>).
/// For automatic handling of the JWT authentication, see the `oauth` feature and the `Message::with_oauth` function.
///
/// This is the `SendMessageRequest` of the Firebase API: it wraps the [MessageBody]
/// together with the `validate_only` flag.
/// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages/send>.
/// Note that the [Debug] implementation redacts the JWT, so that logging a message
/// cannot leak a bearer token.
#[derive(serde::Serialize, Clone)]
#[must_use = "a Message does nothing until it is passed to Client::send"]
pub struct Message {
    #[serde(skip_serializing)]
    pub(crate) project_id: String,
    #[serde(skip_serializing)]
    pub(crate) jwt: String,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    validate_only: bool,
    pub(crate) message: MessageBody,
}

impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Message")
            .field("project_id", &self.project_id)
            .field("jwt", &"[redacted]")
            .field("validate_only", &self.validate_only)
            .field("message", &self.message)
            .finish()
    }
}

impl Message {
    /// Create a new Message.
    pub fn new(project_id: impl Into<String>, jwt: impl Into<String>, body: MessageBody) -> Self {
        Self {
            project_id: project_id.into(),
            jwt: jwt.into(),
            validate_only: false,
            message: body,
        }
    }

    /// Marks the message as a dry run: Firebase validates the request and reports any
    /// errors, but does not deliver anything. Useful for testing message construction
    /// against the real API.
    /// ```rust
    /// use firebae_cm::{Message, MessageBody, Receiver};
    ///
    /// let body = MessageBody::new(Receiver::topic("subscribers"));
    /// let message = Message::new("project_id", "jwt", body).validate_only(true);
    /// ```
    pub fn validate_only(mut self, validate_only: bool) -> Self {
        self.validate_only = validate_only;
        self
    }
}

#[cfg(feature = "oauth")]
mod oauth {
    use std::sync::Arc;

    pub use gcp_auth::Error;
    use gcp_auth::TokenProvider;
    use tokio::sync::OnceCell;

    use super::{Message, MessageBody};

    static TOKEN_PROVIDER: OnceCell<Arc<dyn TokenProvider>> = OnceCell::const_new();

    async fn token_provider() -> Result<&'static Arc<dyn TokenProvider>, Error> {
        TOKEN_PROVIDER.get_or_try_init(gcp_auth::provider).await
    }

    impl Message {
        #[cfg_attr(docsrs, doc(cfg(feature = "oauth")))]
        /// Create a new Message and automatically handle (oauth) authentication. Requires the `oauth` feature.
        /// To use this, make sure to set the `GOOGLE_APPLICATION_CREDENTIALS` environment variable with the path to the `credentials.json` file.
        /// Check the GCP documentation to obtain such file: <https://cloud.google.com/docs/authentication/provide-credentials-adc>.
        ///
        /// The provider caches tokens for their lifetime, so this is cheap to call
        /// repeatedly.
        pub async fn with_oauth(
            project_id: impl Into<String>,
            body: MessageBody,
        ) -> Result<Self, Error> {
            let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
            let token = token_provider().await?.token(scopes).await?;

            Ok(Self::new(project_id, token.as_str(), body))
        }
    }
}
