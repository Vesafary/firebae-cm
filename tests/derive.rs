//! Tests for the `AsFirebaseMap` derive macro re-exported from `firebae-derive`.
//!
//! FCM data payloads are `map<string, string>`, so every field is stringified.

use std::collections::HashMap;

use firebae_cm::{AsFirebaseMap, FirebaseMap, FirebaseMapValue, IntoFirebaseMap};

fn map_of(value: &impl IntoFirebaseMap) -> HashMap<String, String> {
    value.as_map().into_inner()
}

fn entry(map: &HashMap<String, String>, key: &str) -> String {
    map.get(key)
        .unwrap_or_else(|| panic!("missing key {key}"))
        .clone()
}

#[derive(AsFirebaseMap)]
struct Scalars {
    text: String,
    count: i32,
    ratio: f64,
    flag: bool,
    letter: char,
}

#[test]
fn scalar_fields_are_stringified() {
    let map = map_of(&Scalars {
        text: "hello".to_string(),
        count: -5481,
        ratio: 1.5,
        flag: true,
        letter: 'x',
    });

    assert_eq!(map.len(), 5);
    assert_eq!(entry(&map, "text"), "hello");
    assert_eq!(entry(&map, "count"), "-5481");
    assert_eq!(entry(&map, "ratio"), "1.5");
    assert_eq!(entry(&map, "flag"), "true");
    assert_eq!(entry(&map, "letter"), "x");
}

#[derive(AsFirebaseMap)]
struct WithOptions {
    always: String,
    maybe_text: Option<String>,
    maybe_number: Option<u64>,
}

#[test]
fn none_fields_are_omitted_entirely() {
    let map = map_of(&WithOptions {
        always: "here".to_string(),
        maybe_text: None,
        maybe_number: None,
    });

    assert_eq!(map.len(), 1);
    assert_eq!(entry(&map, "always"), "here");
    assert!(!map.contains_key("maybe_text"));
    assert!(!map.contains_key("maybe_number"));
}

#[test]
fn some_fields_are_unwrapped_not_debug_formatted() {
    let map = map_of(&WithOptions {
        always: "here".to_string(),
        maybe_text: Some("value".to_string()),
        maybe_number: Some(42),
    });

    assert_eq!(map.len(), 3);
    assert_eq!(entry(&map, "maybe_text"), "value");
    assert_eq!(entry(&map, "maybe_number"), "42");
}

#[derive(AsFirebaseMap)]
struct FullyQualifiedOption {
    value: std::option::Option<String>,
}

#[test]
fn option_is_detected_through_a_qualified_path() {
    let map = map_of(&FullyQualifiedOption { value: None });
    assert!(map.is_empty());

    let map = map_of(&FullyQualifiedOption {
        value: Some("set".to_string()),
    });
    assert_eq!(entry(&map, "value"), "set");
}

#[derive(AsFirebaseMap)]
struct Generic<T> {
    inner: T,
}

#[test]
fn generic_structs_derive_with_a_map_value_bound() {
    assert_eq!(entry(&map_of(&Generic { inner: 7u8 }), "inner"), "7");
    assert_eq!(
        entry(
            &map_of(&Generic {
                inner: "text".to_string()
            }),
            "inner"
        ),
        "text"
    );
}

struct Custom;

impl std::fmt::Display for Custom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "custom-display")
    }
}

#[derive(AsFirebaseMap)]
struct WithCustomType {
    custom: Custom,
}

#[test]
fn any_display_type_is_usable_as_a_map_value() {
    assert_eq!(
        entry(&map_of(&WithCustomType { custom: Custom }), "custom"),
        "custom-display"
    );
    assert_eq!(Custom.to_firebase_value(), "custom-display");
}

#[test]
fn derive_works_on_a_struct_declared_inside_a_function() {
    #[derive(AsFirebaseMap)]
    struct Local {
        field: String,
    }

    let map = map_of(&Local {
        field: "scoped".to_string(),
    });
    assert_eq!(entry(&map, "field"), "scoped");
}

#[test]
fn manual_impls_can_build_a_map_directly() {
    struct Manual {
        a: String,
        b: i32,
    }

    impl IntoFirebaseMap for Manual {
        fn as_map(&self) -> FirebaseMap {
            let mut m = FirebaseMap::new();
            m.insert("a", &self.a);
            m.insert("b", &self.b);
            m
        }
    }

    let map = map_of(&Manual {
        a: "x".to_string(),
        b: 1,
    });
    assert_eq!(entry(&map, "a"), "x");
    assert_eq!(entry(&map, "b"), "1");
}
