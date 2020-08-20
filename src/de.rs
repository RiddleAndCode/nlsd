use super::error::{Error, Result};
use super::parser::{
    parse_next, parse_number, parse_string, parse_token, Number, ParseError, ParseResult, Parsed,
};
use serde::de;

const TRUE: &str = "true";
const FALSE: &str = "false";
const ON: &str = "on";
const OFF: &str = "off";
const ENABLED: &str = "enabled";
const DISABLED: &str = "disabled";

const EMPTY: &str = "empty";
const NOTHING: &str = "nothing";

const THE: &str = "the";
const OBJECT: &str = "object";
const LIST: &str = "list";
const HENCEFORTH: &str = "henceforth";
const WHERE: &str = "where";
const AN: &str = "an";
const ITEM: &str = "item";
const OF: &str = "of";
const IS: &str = "is";
const AND: &str = "and";
const ANOTHER: &str = "another";

pub struct Deserializer<'de> {
    src: &'de str,
    index: usize,
}

fn unescape_str(string: &str) -> String {
    string.replace(r#"\`"#, r#"`"#)
}

impl<'de> Deserializer<'de> {
    pub fn from_str(src: &'de str) -> Self {
        Self { src, index: 0 }
    }

    fn peek_next(&self) -> Result<Parsed<'de>> {
        let (_, parsed, _) =
            parse_next(&self.src[self.index..]).map_err(|err| self.inc_err_index(err.into()))?;
        Ok(parsed)
    }

    fn parse_next(&mut self) -> Result<Parsed<'de>> {
        self.inc_parse_result(parse_next(&self.src[self.index..]))
    }

    fn parse_token(&mut self) -> Result<&'de str> {
        self.inc_parse_result(parse_token(&self.src[self.index..]))
    }

    fn parse_string(&mut self) -> Result<&'de str> {
        self.inc_parse_result(parse_string(&self.src[self.index..]))
    }

    fn parse_number(&mut self) -> Result<Number> {
        self.inc_parse_result(parse_number(&self.src[self.index..]))
    }

    fn inc_parse_result<T>(&mut self, result: ParseResult<T>) -> Result<T> {
        let (_, parsed, rest) = result.map_err(|err| self.inc_err_index(err.into()))?;
        self.index += self.src.len() - rest.len();
        Ok(parsed)
    }

    fn inc_err_index(&self, err: Error) -> Error {
        match err {
            Error::Parse(err) => Error::Parse(match err {
                ParseError::InvalidString(i) => ParseError::InvalidString(i + self.index),
                ParseError::InvalidNumber(i) => ParseError::InvalidNumber(i + self.index),
                ParseError::ExpectedWhitespace(i) => ParseError::ExpectedWhitespace(i + self.index),
                err => err,
            }),
            err => err,
        }
    }
}

impl<'a, 'de> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.peek_next()? {
            Parsed::Token(token) => match token {
                TRUE | FALSE | ON | OFF | ENABLED | DISABLED => self.deserialize_bool(visitor),
                EMPTY | NOTHING => self.deserialize_unit(visitor),
                _ => Err(Error::Unimplemented),
            },
            Parsed::Number(Number::Float(_)) => self.deserialize_f64(visitor),
            Parsed::Number(Number::Integer(_)) => self.deserialize_i64(visitor),
            Parsed::Str(_) => self.deserialize_str(visitor),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.parse_token()? {
            TRUE | ON | ENABLED => visitor.visit_bool(true),
            FALSE | OFF | DISABLED => visitor.visit_bool(false),
            _ => Err(Error::ExpectedBool),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.parse_token()? {
            EMPTY | NOTHING => visitor.visit_unit(),
            _ => Err(Error::ExpectedNull),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.parse_number()? {
            Number::Integer(num) => visitor.visit_i64(num),
            Number::Float(num) => {
                if num.trunc() == num {
                    visitor.visit_i64(num as i64)
                } else {
                    Err(Error::ExpectedInteger)
                }
            }
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.parse_number()? {
            Number::Integer(num) => visitor.visit_i32(num as i32),
            Number::Float(num) => {
                if num.trunc() == num {
                    visitor.visit_i32(num as i32)
                } else {
                    Err(Error::ExpectedInteger)
                }
            }
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.parse_number()? {
            Number::Integer(num) => visitor.visit_i16(num as i16),
            Number::Float(num) => {
                if num.trunc() == num {
                    visitor.visit_i16(num as i16)
                } else {
                    Err(Error::ExpectedInteger)
                }
            }
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.parse_number()? {
            Number::Integer(num) => visitor.visit_i8(num as i8),
            Number::Float(num) => {
                if num.trunc() == num {
                    visitor.visit_i8(num as i8)
                } else {
                    Err(Error::ExpectedInteger)
                }
            }
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.parse_number()? {
            Number::Integer(num) => {
                if num.is_positive() {
                    visitor.visit_u64(num as u64)
                } else {
                    Err(Error::ExpectedUnsigned)
                }
            }
            Number::Float(num) => {
                if num.trunc() == num {
                    if num.is_sign_positive() {
                        visitor.visit_u64(num as u64)
                    } else {
                        Err(Error::ExpectedUnsigned)
                    }
                } else {
                    Err(Error::ExpectedInteger)
                }
            }
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.parse_number()? {
            Number::Integer(num) => {
                if num.is_positive() {
                    visitor.visit_u32(num as u32)
                } else {
                    Err(Error::ExpectedUnsigned)
                }
            }
            Number::Float(num) => {
                if num.trunc() == num {
                    if num.is_sign_positive() {
                        visitor.visit_u32(num as u32)
                    } else {
                        Err(Error::ExpectedUnsigned)
                    }
                } else {
                    Err(Error::ExpectedInteger)
                }
            }
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.parse_number()? {
            Number::Integer(num) => {
                if num.is_positive() {
                    visitor.visit_u16(num as u16)
                } else {
                    Err(Error::ExpectedUnsigned)
                }
            }
            Number::Float(num) => {
                if num.trunc() == num {
                    if num.is_sign_positive() {
                        visitor.visit_u16(num as u16)
                    } else {
                        Err(Error::ExpectedUnsigned)
                    }
                } else {
                    Err(Error::ExpectedInteger)
                }
            }
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.parse_number()? {
            Number::Integer(num) => {
                if num.is_positive() {
                    visitor.visit_u8(num as u8)
                } else {
                    Err(Error::ExpectedUnsigned)
                }
            }
            Number::Float(num) => {
                if num.trunc() == num {
                    if num.is_sign_positive() {
                        visitor.visit_u8(num as u8)
                    } else {
                        Err(Error::ExpectedUnsigned)
                    }
                } else {
                    Err(Error::ExpectedInteger)
                }
            }
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.parse_number()? {
            Number::Integer(num) => visitor.visit_f64(num as f64),
            Number::Float(num) => visitor.visit_f64(num),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.parse_number()? {
            Number::Integer(num) => visitor.visit_f32(num as f32),
            Number::Float(num) => visitor.visit_f32(num as f32),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_string(unescape_str(self.parse_string()?))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut chars = self.parse_string()?.chars();
        let ch = if let Some(ch) = chars.next() {
            ch
        } else {
            return Err(Error::ExpectedChar);
        };
        if chars.next().is_some() {
            return Err(Error::ExpectedChar);
        }
        visitor.visit_char(ch)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.peek_next()? {
            Parsed::Token(string) => match string {
                EMPTY | NOTHING => {
                    let _ = self.parse_next()?;
                    return visitor.visit_none();
                }
                _ => (),
            },
            _ => (),
        }
        visitor.visit_some(self)
    }

    fn deserialize_unit_struct<V>(
        self,
        _: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_bytes<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        return Err(Error::Unimplemented);
    }

    fn deserialize_byte_buf<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        return Err(Error::Unimplemented);
    }

    serde::forward_to_deserialize_any! {
        seq tuple tuple_struct map struct enum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::*;
    use serde_json::{json, Value};

    #[test]
    fn deserialize_bool() -> Result<()> {
        assert_eq!(true, from_str::<bool>("true")?);
        assert_eq!(false, from_str::<bool>("false")?);
        assert_eq!(true, from_str::<bool>("on")?);
        assert_eq!(false, from_str::<bool>("off")?);
        assert_eq!(true, from_str::<bool>("enabled")?);
        assert_eq!(false, from_str::<bool>("disabled")?);

        assert_eq!(json!(true), from_str::<Value>("true")?);
        assert_eq!(json!(false), from_str::<Value>("false")?);
        assert_eq!(json!(true), from_str::<Value>("on")?);
        assert_eq!(json!(false), from_str::<Value>("off")?);
        assert_eq!(json!(true), from_str::<Value>("enabled")?);
        assert_eq!(json!(false), from_str::<Value>("disabled")?);

        Ok(())
    }

    #[test]
    fn deserialize_unit() -> Result<()> {
        assert_eq!((), from_str::<()>("empty")?);
        assert_eq!((), from_str::<()>("nothing")?);

        assert_eq!(json!(null), from_str::<Value>("empty")?);
        assert_eq!(json!(null), from_str::<Value>("nothing")?);

        Ok(())
    }

    #[test]
    fn deserialize_number() -> Result<()> {
        assert_eq!(1.2, from_str::<f64>("1.2")?);
        assert_eq!(-1.2, from_str::<f64>("-1.2")?);
        assert_eq!(1.2, from_str::<f32>("1.2")?);
        assert_eq!(-1.2, from_str::<f32>("-1.2")?);

        assert_eq!(1, from_str::<i64>("1")?);
        assert_eq!(-1, from_str::<i64>("-1")?);
        assert_eq!(1, from_str::<i32>("1")?);
        assert_eq!(-1, from_str::<i32>("-1")?);
        assert_eq!(1, from_str::<i16>("1")?);
        assert_eq!(-1, from_str::<i16>("-1")?);
        assert_eq!(1, from_str::<i8>("1")?);
        assert_eq!(-1, from_str::<i8>("-1")?);

        assert_eq!(1, from_str::<u64>("1")?);
        assert_eq!(1, from_str::<u32>("1")?);
        assert_eq!(1, from_str::<u16>("1")?);
        assert_eq!(1, from_str::<u8>("1")?);

        assert_eq!(json!(1), from_str::<Value>("1")?);
        assert_eq!(json!(1.2), from_str::<Value>("1.2")?);
        assert_eq!(json!(-1), from_str::<Value>("-1")?);
        assert_eq!(json!(-1.2), from_str::<Value>("-1.2")?);

        Ok(())
    }

    #[test]
    fn deserialize_str() -> Result<()> {
        assert_eq!("hello", from_str::<String>("`hello`")?);
        assert_eq!("hello", from_str::<String>("`hello`")?);
        assert_eq!(
            "escaped`string",
            from_str::<String>(r#"`escaped\`string`"#)?
        );
        assert_eq!(json!("hello"), from_str::<Value>("`hello`")?);
        Ok(())
    }

    #[test]
    fn deserialize_char() -> Result<()> {
        assert_eq!('a', from_str::<char>("`a`")?);
        assert_eq!(json!("a"), from_str::<Value>("`a`")?);
        Ok(())
    }

    #[test]
    fn deserialize_option() -> Result<()> {
        assert_eq!(
            Some("hello".to_string()),
            from_str::<Option<String>>("`hello`")?
        );
        assert_eq!(Some(123), from_str::<Option<i64>>("123")?);
        assert_eq!(Some(123.123), from_str::<Option<f64>>("123.123")?);
        assert_eq!(None, from_str::<Option<i64>>("empty")?);
        assert_eq!(None, from_str::<Option<i64>>("nothing")?);
        Ok(())
    }
}
