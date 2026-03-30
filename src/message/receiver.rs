/// Represents a receiver of your message.
/// Can be a Token (for a single device),
/// a Topic (for all devices that have subscribed to that topic),
/// or a Condition (for all devices that meet the condition).
/// See the fields in [the documentation](https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#resource:-message).
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Receiver {
    Token(String),
    Topic(String),
    Condition(String),
}

impl Receiver {
    /// Create a Token variant with anything that implements `Into<String>` (such as `&str`).
    /// ```rust
    /// let token = Receiver::token("abcd");
    /// ```
    pub fn token(token: impl Into<String>) -> Self {
        Self::Token(token.into())
    }

    /// Create a Topic variant with anything that implements `Into<String>` (such as `&str`).
    /// ```rust
    /// let topic = Receiver::topic("abcd");
    /// ```
    pub fn topic(topic: impl Into<String>) -> Self {
        Self::Topic(topic.into())
    }

    /// Create a Condition variant with anything that implements `Into<String>` (such as `&str`).
    /// ```rust
    /// let condition = Receiver::condition("abcd");
    /// ```
    pub fn condition(condition: impl Into<String>) -> Self {
        Self::Condition(condition.into())
    }
}
