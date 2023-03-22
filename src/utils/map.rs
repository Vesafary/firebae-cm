use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct FirebaseMap(HashMap<String, String>);

/// Represents a String: String map that can be used for data, headers, etc fields within Firebase objects.
impl FirebaseMap {
    /// Create a new map.
    pub fn new() -> Self {
        Default::default()
    }

    /// Insert an entry into the map. The key must implement Into<String> (this will usually be the field name),
    /// the value must implement FirebaseMapValue. This struct is usually used together with the [IntoFirebaseMap] trait.
    /// Also, check out the AsFirebaseMap trait for a derive implementation.
    /// ```rust
    /// use firebae_cm::{FirebaseMap, IntoFirebaseMap};
    /// struct Data {
    ///     a: String,
    ///     b: i32,
    /// }
    ///
    /// impl IntoFirebaseMap for Data {
    ///     fn as_map(self) -> FirebaseMap {
    ///         let mut m = FirebaseMap::new();
    ///         m.insert("a", self.a);
    ///         m.insert("b", self.b);
    ///         m
    ///     }
    /// }
    /// ```
    pub fn insert(&mut self, key: impl Into<String>, value: &impl FirebaseMapValue) {
        self.0.insert(key.into(), value.to_string());
    }

    pub(crate) fn get_map(self) -> HashMap<String, String> {
        self.0
    }
}

/// Trait to convert a struct into a [FirebaseMap].
pub trait IntoFirebaseMap {
    fn as_map(&self) -> FirebaseMap;
}

/// Trait to convert a value into a String.
pub trait FirebaseMapValue {
    fn to_string(&self) -> String;
}

impl FirebaseMapValue for String {
    fn to_string(&self) -> String {
        self.clone()
    }
}

macro_rules! disp_impl {
    ($typename: ident) => {
        impl FirebaseMapValue for $typename {
            fn to_string(&self) -> String {
                format!("{}", self)
            }
        }
    };
}

disp_impl!(i8);
disp_impl!(i16);
disp_impl!(i32);
disp_impl!(i64);
disp_impl!(i128);
disp_impl!(isize);

disp_impl!(u8);
disp_impl!(u16);
disp_impl!(u32);
disp_impl!(u64);
disp_impl!(u128);
disp_impl!(usize);

disp_impl!(bool);
disp_impl!(char);
disp_impl!(f32);
disp_impl!(f64);
