/// Represents a receiver of your message.
/// Can be a Token or Fid (for a single device),
/// a Topic (for all devices that have subscribed to that topic),
/// or a Condition (for all devices that meet the condition).
/// See the fields in [the documentation](https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#resource:-message).
///
/// This maps onto the required `target` union of the message, so exactly one
/// variant is sent.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[must_use]
pub enum Receiver {
    /// A registration token.
    ///
    /// Firebase has deprecated this field in favour of [`Receiver::Fid`]. It still
    /// works, and during the transition period it also accepts a Firebase
    /// Installation ID, so existing code keeps functioning. Prefer [`Receiver::Fid`]
    /// for new code.
    Token(String),
    /// A Firebase Installation ID (FID). The replacement for [`Receiver::Token`].
    Fid(String),
    /// A topic name, e.g. `"weather"`. The `/topics/` prefix should not be included.
    Topic(String),
    /// A condition, e.g. `"'foo' in topics && 'bar' in topics"`.
    Condition(String),
}

impl Receiver {
    /// Create a Token variant with anything that implements `Into<String>` (such as `&str`).
    ///
    /// Firebase has deprecated registration tokens in favour of Firebase Installation
    /// IDs; see [`Receiver::fid`].
    /// ```rust
    /// use firebae_cm::Receiver;
    ///
    /// let token = Receiver::token("abcd");
    /// ```
    pub fn token(token: impl Into<String>) -> Self {
        Self::Token(token.into())
    }

    /// Create a Fid variant with anything that implements `Into<String>` (such as `&str`).
    /// ```rust
    /// use firebae_cm::Receiver;
    ///
    /// let fid = Receiver::fid("abcd");
    /// ```
    pub fn fid(fid: impl Into<String>) -> Self {
        Self::Fid(fid.into())
    }

    /// Create a Topic variant with anything that implements `Into<String>` (such as `&str`).
    /// ```rust
    /// use firebae_cm::Receiver;
    ///
    /// let topic = Receiver::topic("abcd");
    /// ```
    pub fn topic(topic: impl Into<String>) -> Self {
        Self::Topic(topic.into())
    }

    /// Create a Condition variant with anything that implements `Into<String>` (such as `&str`).
    /// ```rust
    /// use firebae_cm::Receiver;
    ///
    /// let condition = Receiver::condition("abcd");
    /// ```
    pub fn condition(condition: impl Into<String>) -> Self {
        Self::Condition(condition.into())
    }
}
