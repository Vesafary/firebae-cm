use std::collections::HashMap;
use std::fmt::Display;

/// Represents a String: String map that can be used in some Firebase objects for fields such as data, payload and headers.
/// You can use the [crate::AsFirebaseMap] derive implementation for ease.
/// ```rust
/// use firebae_cm::AsFirebaseMap;
///
/// #[derive(AsFirebaseMap)]
/// struct Data {
///     a: String,
///     b: i32,  // Transformed into a String.
/// }
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FirebaseMap(HashMap<String, String>);

impl FirebaseMap {
    /// Create a new map.
    pub fn new() -> Self {
        Default::default()
    }

    /// Insert an entry into the map. The key must implement `Into<String>` (this will usually be the field name),
    /// the value must implement [FirebaseMapValue], which is satisfied by anything implementing [Display].
    /// This struct is usually used together with the [IntoFirebaseMap] trait.
    /// ```rust
    /// use firebae_cm::{FirebaseMap, IntoFirebaseMap};
    ///
    /// struct Data {
    ///     a: String,
    ///     b: i32,
    /// }
    ///
    /// impl IntoFirebaseMap for Data {
    ///     fn as_map(&self) -> FirebaseMap {
    ///         let mut m = FirebaseMap::new();
    ///         m.insert("a", &self.a);
    ///         m.insert("b", &self.b);
    ///         m
    ///     }
    /// }
    /// ```
    pub fn insert(&mut self, key: impl Into<String>, value: &impl FirebaseMapValue) {
        self.0.insert(key.into(), value.to_firebase_value());
    }

    /// Consumes the map and returns the underlying `HashMap`.
    pub fn into_inner(self) -> HashMap<String, String> {
        self.0
    }

    /// Checks the keys against the reserved words FCM disallows in a `data` payload.
    ///
    /// Firebase would reject such a message with `INVALID_ARGUMENT`, so catching it
    /// locally saves a round trip. Only `data` is restricted this way — `headers` and
    /// `payload` are not.
    /// See <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#resource:-message>.
    pub fn validate_as_data(&self) -> crate::Result<()> {
        for key in self.0.keys() {
            let reserved = key == "from"
                || key == "message_type"
                || key.starts_with("google.")
                || key.starts_with("gcm.notification.");

            if reserved {
                return Err(crate::Error::ReservedDataKey(key.clone()));
            }
        }
        Ok(())
    }
}

/// Trait to convert a struct into a [FirebaseMap].
pub trait IntoFirebaseMap {
    /// Builds the string map Firebase expects for this value.
    fn as_map(&self) -> FirebaseMap;
}

/// Trait to convert a value into the String that Firebase expects.
///
/// Blanket-implemented for every type implementing [Display], which covers the
/// primitives and `String`. To use your own type as a map value, implement
/// [Display] for it.
pub trait FirebaseMapValue {
    /// Renders this value as the String that Firebase stores.
    fn to_firebase_value(&self) -> String;
}

impl<T: Display> FirebaseMapValue for T {
    fn to_firebase_value(&self) -> String {
        self.to_string()
    }
}
