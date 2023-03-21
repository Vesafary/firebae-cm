use crate::MessageBody;

/// A representation of a complete message that can be sent. It requires your project_id and an authentication JWT token (see <https://cloud.google.com/docs/authentication/>).
/// For automatic handling of the JWT authentication, see the `oauth` feature and the `Message::with_oauth` function.
#[derive(serde::Serialize, Debug)]
pub struct Message {
    #[serde(skip_serializing)]
    pub(crate) project_id: String,
    #[serde(skip_serializing)]
    pub(crate) jwt: String,
    pub(crate) message: MessageBody,
}

impl Message {
    /// Create a new Message.
    pub fn new(project_id: impl Into<String>, jwt: impl Into<String>, body: MessageBody) -> Self {
        Self {
            project_id: project_id.into(),
            jwt: jwt.into(),
            message: body,
        }
    }
}

#[cfg(all(feature = "oauth", not(all(loom, test))))]
mod oauth {
    use gcp_auth::AuthenticationManager;
    pub use gcp_auth::Error;
    use tokio::sync::OnceCell;

    use super::{Message, MessageBody};

    static AUTH_MANAGER: OnceCell<AuthenticationManager> = OnceCell::const_new();

    async fn authentication_manager() -> &'static AuthenticationManager {
        AUTH_MANAGER
            .get_or_init(|| async {
                AuthenticationManager::new()
                    .await
                    .expect("unable to initialize authentication manager")
            })
            .await
    }

    impl Message {
        #[cfg_attr(docsrs, doc(cfg(feature = "oauth")))]
        /// Create a new Message and automatically handle (oauth) authentication. Requires the `oauth` feature.
        /// To use this, make sure to set the `GOOGLE_APPLICATION_CREDENTIALS` environment variable with the path to the `credentials.json` file.
        /// Check the GCP documentation to obtain such file: <https://cloud.google.com/docs/authentication/provide-credentials-adc>.
        pub async fn with_oauth(
            project_id: impl Into<String>,
            body: MessageBody,
        ) -> Result<Self, Error> {
            let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
            let full_token = authentication_manager().await.get_token(scopes).await?;
            let token = full_token.as_str().trim_end_matches(".");
    
            Ok(Self::new(project_id, token, body))
        }
    }
}
