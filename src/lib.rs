//! String deserializers customization for serde.

#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

extern crate alloc;

mod string;
mod string_non_empty;

pub use crate::{
    string::string,
    string_non_empty::{option_string_non_empty, string_non_empty},
};
