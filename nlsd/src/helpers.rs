use crate::de::Deserializer;
use crate::error::Result;
use crate::ser::Serializer;
use alloc::fmt::Write;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

/// deserialize an instance of `T` from NLSD text
pub fn from_str<'de, T>(s: &'de str) -> Result<T>
where
    T: Deserialize<'de>,
{
    let mut deserializer = Deserializer::from_str(s);
    T::deserialize(&mut deserializer)
}

/// deserialize an instance of `T` from NLSD bytes
pub fn from_slice<'de, T>(s: &'de [u8]) -> Result<T>
where
    T: Deserialize<'de>,
{
    from_str(core::str::from_utf8(s)?)
}

/// serialize an instance of `T` to a string
pub fn to_string<T: ?Sized>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut writer = String::new();
    to_writer(&mut writer, value)?;
    Ok(writer)
}

/// serialize an instance of `T` to bytes
pub fn to_vec<T: ?Sized>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    Ok(to_string(value)?.into_bytes())
}

/// serialize an instance of `T` to a writer
pub fn to_writer<W, T: ?Sized>(writer: W, value: &T) -> Result<()>
where
    W: Write,
    T: Serialize,
{
    let mut ser = Serializer::new(writer);
    value.serialize(&mut ser)?;
    Ok(())
}
