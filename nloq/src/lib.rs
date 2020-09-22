//! Parses natural language to produce a `Vec<Query>`. The main entrypoints are
//! `from_slice` and `from_str`
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std as core;

extern crate alloc;
pub extern crate object_query;

mod de;
mod helpers;

pub use de::Deserializer;
pub use helpers::*;
