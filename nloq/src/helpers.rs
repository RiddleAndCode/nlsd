use crate::de::Deserializer;
use object_query::Query;
use std::str::{from_utf8, Utf8Error};

/// parse an NLOQ query from a string slice
pub fn from_str<'a>(s: &'a str) -> Vec<Query<'a>> {
    Deserializer::from_str(s).query()
}

/// parse an NLOQ query from a byte string slice
pub fn from_slice<'a>(s: &'a [u8]) -> Result<Vec<Query<'a>>, Utf8Error> {
    Ok(from_str(from_utf8(s)?))
}
