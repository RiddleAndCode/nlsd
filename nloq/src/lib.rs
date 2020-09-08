//! Parses natural language to produce a `Vec<Query>`. The main entrypoints are
//! `from_slice` and `from_str`

pub extern crate object_query;

mod de;
mod helpers;

pub use de::Deserializer;
pub use helpers::*;
