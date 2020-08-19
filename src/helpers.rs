use crate::de::Deserializer;
use crate::error::Result;
use crate::ser::Serializer;
use serde::{Deserialize, Serialize};
use std::io::Write;

pub fn from_str<'de, T>(s: &'de str) -> Result<T>
where
    T: Deserialize<'de>,
{
    let mut deserializer = Deserializer::from_str(s);
    T::deserialize(&mut deserializer)
}

pub fn to_string<T: ?Sized>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let vec = to_vec(value)?;
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}

pub fn to_vec<T: ?Sized>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value)?;
    Ok(writer)
}

pub fn to_writer<W, T: ?Sized>(writer: W, value: &T) -> Result<()>
where
    W: Write,
    T: Serialize,
{
    let mut ser = Serializer::new(writer);
    value.serialize(&mut ser)?;
    Ok(())
}
