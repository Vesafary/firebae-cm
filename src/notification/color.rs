/// Represents a color as described [in the documentation](https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#color).
///
/// Every component is optional and in the interval `[0, 1]`. Omitted components are
/// not serialized at all, which matters most for `alpha`: FCM renders a color with an
/// absent `alpha` as fully opaque, whereas an explicit `0.0` means fully transparent.
///
/// ```rust
/// use firebae_cm::Color;
///
/// let red = Color::rgb(1.0, 0.0, 0.0);
/// let half_transparent_red = Color::rgb(1.0, 0.0, 0.0).with_alpha(0.5);
/// ```
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone, PartialEq)]
#[must_use]
pub struct Color {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The amount of red, in the interval `[0, 1]`.
    pub red: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The amount of green, in the interval `[0, 1]`.
    pub green: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The amount of blue, in the interval `[0, 1]`.
    pub blue: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The opacity, in the interval `[0, 1]`, where `1.0` is solid and `0.0` fully
    /// transparent. When absent, Firebase renders the color as fully opaque.
    pub alpha: Option<f32>,
}

impl Color {
    /// Creates a color with no components set. FCM renders this as fully opaque black.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates an opaque color. Each component is in the interval `[0, 1]`.
    pub fn rgb(red: f32, green: f32, blue: f32) -> Self {
        Self {
            red: Some(red),
            green: Some(green),
            blue: Some(blue),
            alpha: None,
        }
    }

    /// Sets the alpha component, in the interval `[0, 1]`, where `0.0` is fully
    /// transparent and `1.0` fully opaque.
    pub fn with_alpha(mut self, alpha: f32) -> Self {
        self.alpha = Some(alpha);
        self
    }
}
