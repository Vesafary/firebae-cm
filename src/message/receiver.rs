#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Receiver {
    Token(String),
    Topic(String),
    Condition(String),
}
