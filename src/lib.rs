//! **De**serialization **trim**ming for strings in serde models.

#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

extern crate alloc;

mod string;
mod string_non_empty;
mod vec_string;

pub use crate::{
    string::string,
    string_non_empty::{option_string_non_empty, string_non_empty},
    vec_string::vec_string,
};
