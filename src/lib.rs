//! TODO

#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

extern crate alloc;

use alloc::{borrow::ToOwned, string::String};

use serde::{de, Deserialize as _, Deserializer};

pub fn string<'a, D: Deserializer<'a>>(de: D) -> Result<String, D::Error> {
    String::deserialize(de).map(|val| val.trim().to_owned())
}

pub fn string_non_empty<'a, D: Deserializer<'a>>(de: D) -> Result<String, D::Error> {
    match String::deserialize(de) {
        Ok(val) if val.trim().is_empty() => Err(de::Error::invalid_value(
            de::Unexpected::Other("empty string"),
            &"non-empty string",
        )),
        Ok(val) => Ok(val.trim().to_owned()),
        Err(_) => todo!(),
    }
}

pub fn option_string_non_empty<'a, D: Deserializer<'a>>(de: D) -> Result<Option<String>, D::Error> {
    String::deserialize(de).map(|val| {
        Some(val.trim())
            .filter(|val| !val.is_empty())
            .map(ToOwned::to_owned)
    })
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[test]
    fn string() {
        #[derive(Debug, Deserialize, PartialEq, Eq)]
        struct Foo {
            #[serde(deserialize_with = "crate::string")]
            foo: String,
        }

        impl Foo {
            fn new(foo: impl Into<String>) -> Self {
                Self { foo: foo.into() }
            }
        }

        assert_eq!(
            Foo::new(""),
            serde_json::from_str(r#"{ "foo": "" }"#).unwrap(),
        );
        assert_eq!(
            Foo::new(""),
            serde_json::from_str(r#"{ "foo": " " }"#).unwrap(),
        );
        assert_eq!(
            Foo::new("bar"),
            serde_json::from_str(r#"{ "foo": "bar" }"#).unwrap(),
        );
        assert_eq!(
            Foo::new("bar"),
            serde_json::from_str(r#"{ "foo": " bar" }"#).unwrap(),
        );
        assert_eq!(
            Foo::new("bar"),
            serde_json::from_str(r#"{ "foo": "  bar" }"#).unwrap(),
        );
        assert_eq!(
            Foo::new("bar"),
            serde_json::from_str(r#"{ "foo": "bar " }"#).unwrap(),
        );
        assert_eq!(
            Foo::new("bar"),
            serde_json::from_str(r#"{ "foo": "  bar  " }"#).unwrap(),
        );
    }

    #[test]
    fn option_string_non_empty() {
        #[derive(Debug, Deserialize, PartialEq, Eq)]
        struct Foo {
            #[serde(deserialize_with = "crate::option_string_non_empty")]
            foo: Option<String>,
        }

        impl Foo {
            fn new(foo: impl Into<String>) -> Self {
                Self {
                    foo: Some(foo.into()),
                }
            }

            fn none() -> Self {
                Self { foo: None }
            }
        }

        assert_eq!(
            Foo::none(),
            serde_json::from_str(r#"{ "foo": "" }"#).unwrap(),
        );
        assert_eq!(
            Foo::none(),
            serde_json::from_str(r#"{ "foo": "  " }"#).unwrap(),
        );
        assert_eq!(
            Foo::new("bar"),
            serde_json::from_str(r#"{ "foo": " bar" }"#).unwrap(),
        );
        assert_eq!(
            Foo::new("bar"),
            serde_json::from_str(r#"{ "foo": "  bar" }"#).unwrap(),
        );
        assert_eq!(
            Foo::new("bar"),
            serde_json::from_str(r#"{ "foo": "bar " }"#).unwrap(),
        );
        assert_eq!(
            Foo::new("bar"),
            serde_json::from_str(r#"{ "foo": "  bar  " }"#).unwrap(),
        );
    }
}
