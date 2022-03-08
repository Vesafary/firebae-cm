use crate::MessageBody;

#[derive(serde::Serialize, Debug)]
pub struct Message {
    #[serde(skip_serializing)]
    pub project_id: String,
    #[serde(skip_serializing)]
    pub jwt: String,
    pub message: MessageBody,
}

impl Message {
    pub fn new(project_id: impl Into<String>, jwt: impl Into<String>, body: MessageBody) -> Self {
        Self {
            project_id: project_id.into(),
            jwt: jwt.into(),
            message: body,
        }
    }
}
