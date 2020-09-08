//! A Natural Language Structured Document `serde` implementation. The main entrypoints
//! are the `from_str` and `to_string` methods which take deserializable and serializable
//! objects respectively and converts the from and to English. See the
//! [README](https://github.com/RiddleAndCode/nlsd/blob/master/nlsd/README.md) for more information
//! on the specifications.

mod de;
mod error;
mod helpers;
mod ser;

pub use de::Deserializer;
pub use error::{Error, Result};
pub use helpers::*;
pub use ser::Serializer;
