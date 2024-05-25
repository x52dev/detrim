//! **De**serialization **trim**ming for strings in serde models.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

extern crate alloc;

#[cfg(feature = "std")]
mod hashset_string;
mod string;
mod string_non_empty;
mod vec_string;

pub use crate::{
    string::{cow_str, option_string, str, string},
    string_non_empty::{option_string_non_empty, string_non_empty},
    vec_string::vec_string,
};
#[cfg(feature = "std")]
pub use hashset_string::hashset_string;
