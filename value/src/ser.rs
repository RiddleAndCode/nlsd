use crate::value::{Key, Number, Value};
use serde::ser::{self, Error as SerError};

impl ser::Serialize for Key {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match self {
            Key::Bool(b) => serializer.serialize_bool(*b),
            Key::Number(Number::Integer(n)) => serializer.serialize_i64(*n),
            Key::Number(Number::Float(n)) => serializer.serialize_f64(*n),
            Key::String(s) => serializer.serialize_str(s),
        }
    }
}

impl<U> ser::Serialize for Value<U> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match self {
            Value::Null => serializer.serialize_unit(),
            Value::Bool(b) => serializer.serialize_bool(*b),
            Value::Number(Number::Integer(n)) => serializer.serialize_i64(*n),
            Value::Number(Number::Float(n)) => serializer.serialize_f64(*n),
            Value::String(s) => serializer.serialize_str(s),
            _ => Err(S::Error::custom("unimplimented")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit::SimpleValue;
    use nlsd::{to_string, Result};

    #[test]
    fn serialize_key_bool() -> Result<()> {
        assert_eq!("true", to_string(&Key::Bool(true))?);
        assert_eq!("false", to_string(&Key::Bool(false))?);
        Ok(())
    }

    #[test]
    fn serialize_key_num() -> Result<()> {
        assert_eq!("1", to_string(&Key::Number(Number::Integer(1)))?);
        assert_eq!("1.1", to_string(&Key::Number(Number::Float(1.1)))?);
        Ok(())
    }

    #[test]
    fn serialize_key_string() -> Result<()> {
        assert_eq!(
            "`hello, world`",
            to_string(&Key::String("hello, world".to_string()))?
        );
        Ok(())
    }

    #[test]
    fn serialize_unit() -> Result<()> {
        assert_eq!("empty", to_string(&SimpleValue::Null)?);
        Ok(())
    }

    #[test]
    fn serialize_bool() -> Result<()> {
        assert_eq!("true", to_string(&SimpleValue::Bool(true))?);
        assert_eq!("false", to_string(&SimpleValue::Bool(false))?);
        Ok(())
    }

    #[test]
    fn serialize_num() -> Result<()> {
        assert_eq!("1", to_string(&SimpleValue::Number(Number::Integer(1)))?);
        assert_eq!("1.1", to_string(&SimpleValue::Number(Number::Float(1.1)))?);
        Ok(())
    }

    #[test]
    fn serialize_string() -> Result<()> {
        assert_eq!(
            "`hello, world`",
            to_string(&SimpleValue::String("hello, world".to_string()))?
        );
        Ok(())
    }
}
