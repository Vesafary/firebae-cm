/// Represents a color as described [in the documentation](https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#color).
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}
