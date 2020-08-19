use super::error::{Error, Result};
use super::parser::{parse_next, Number, ParseError, Parsed};
use serde::de;

const TRUE_TOKEN: Parsed<'static> = Parsed::Token("true");
const FALSE_TOKEN: Parsed<'static> = Parsed::Token("false");
const ON_TOKEN: Parsed<'static> = Parsed::Token("on");
const OFF_TOKEN: Parsed<'static> = Parsed::Token("off");
const ENABLED_TOKEN: Parsed<'static> = Parsed::Token("enabled");
const DISABLED_TOKEN: Parsed<'static> = Parsed::Token("disabled");

const EMPTY_TOKEN: Parsed<'static> = Parsed::Token("empty");
const NOTHING_TOKEN: Parsed<'static> = Parsed::Token("nothing");

const THE_TOKEN: Parsed<'static> = Parsed::Token("the");
const OBJECT_TOKEN: Parsed<'static> = Parsed::Token("object");
const LIST_TOKEN: Parsed<'static> = Parsed::Token("list");
const HENCEFORTH_TOKEN: Parsed<'static> = Parsed::Token("henceforth");
const WHERE_TOKEN: Parsed<'static> = Parsed::Token("where");
const AN_TOKEN: Parsed<'static> = Parsed::Token("an");
const ITEM_TOKEN: Parsed<'static> = Parsed::Token("item");
const OF_TOKEN: Parsed<'static> = Parsed::Token("of");
const IS_TOKEN: Parsed<'static> = Parsed::Token("is");
const AND_TOKEN: Parsed<'static> = Parsed::Token("and");
const ANOTHER_TOKEN: Parsed<'static> = Parsed::Token("another");

pub struct Deserializer<'de> {
    src: &'de str,
    index: usize,
}

impl<'de> Deserializer<'de> {
    pub fn from_str(src: &'de str) -> Self {
        Self { src, index: 0 }
    }

    fn peek_next(&self) -> Result<Parsed<'de>> {
        let (_, parsed, _) = parse_next(&self.src[self.index..])?;
        Ok(parsed)
    }

    fn parse_next(&mut self) -> Result<Parsed<'de>> {
        let (_, parsed, rest) = parse_next(&self.src[self.index..]).map_err(|err| match err {
            ParseError::InvalidString(i) => ParseError::InvalidString(i + self.index),
            ParseError::InvalidNumber(i) => ParseError::InvalidNumber(i + self.index),
            ParseError::ExpectedWhitespace(i) => ParseError::ExpectedWhitespace(i + self.index),
            err => err,
        })?;
        self.index += self.src.len() - rest.len();
        Ok(parsed)
    }
}

impl<'a, 'de> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.peek_next()? {
            TRUE_TOKEN | FALSE_TOKEN | ON_TOKEN | OFF_TOKEN | ENABLED_TOKEN | DISABLED_TOKEN => {
                self.deserialize_bool(visitor)
            }
            EMPTY_TOKEN | NOTHING_TOKEN => self.deserialize_unit(visitor),
            Parsed::Number(Number::Float(_)) => self.deserialize_f64(visitor),
            Parsed::Number(Number::Integer(_)) => self.deserialize_i64(visitor),
            Parsed::Str(_) => self.deserialize_str(visitor),
            _ => Err(Error::Unimplemented),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.parse_next()? {
            TRUE_TOKEN | ON_TOKEN | ENABLED_TOKEN => visitor.visit_bool(true),
            FALSE_TOKEN | OFF_TOKEN | DISABLED_TOKEN => visitor.visit_bool(false),
            _ => Err(Error::ExpectedBool),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.parse_next()? {
            EMPTY_TOKEN | NOTHING_TOKEN => visitor.visit_unit(),
            _ => Err(Error::ExpectedNull),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.parse_next()? {
            Parsed::Number(Number::Integer(num)) => visitor.visit_i64(num),
            Parsed::Number(Number::Float(num)) => {
                if num.trunc() == num {
                    visitor.visit_i64(num as i64)
                } else {
                    Err(Error::ExpectedInteger)
                }
            }
            _ => Err(Error::ExpectedInteger),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.parse_next()? {
            Parsed::Number(Number::Integer(num)) => visitor.visit_i32(num as i32),
            Parsed::Number(Number::Float(num)) => {
                if num.trunc() == num {
                    visitor.visit_i32(num as i32)
                } else {
                    Err(Error::ExpectedInteger)
                }
            }
            _ => Err(Error::ExpectedInteger),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.parse_next()? {
            Parsed::Number(Number::Integer(num)) => visitor.visit_i16(num as i16),
            Parsed::Number(Number::Float(num)) => {
                if num.trunc() == num {
                    visitor.visit_i16(num as i16)
                } else {
                    Err(Error::ExpectedInteger)
                }
            }
            _ => Err(Error::ExpectedInteger),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.parse_next()? {
            Parsed::Number(Number::Integer(num)) => visitor.visit_i8(num as i8),
            Parsed::Number(Number::Float(num)) => {
                if num.trunc() == num {
                    visitor.visit_i8(num as i8)
                } else {
                    Err(Error::ExpectedInteger)
                }
            }
            _ => Err(Error::ExpectedInteger),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.parse_next()? {
            Parsed::Number(Number::Integer(num)) => {
                if num.is_positive() {
                    visitor.visit_u64(num as u64)
                } else {
                    Err(Error::ExpectedUnsigned)
                }
            }
            Parsed::Number(Number::Float(num)) => {
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
            _ => Err(Error::ExpectedInteger),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.parse_next()? {
            Parsed::Number(Number::Integer(num)) => {
                if num.is_positive() {
                    visitor.visit_u32(num as u32)
                } else {
                    Err(Error::ExpectedUnsigned)
                }
            }
            Parsed::Number(Number::Float(num)) => {
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
            _ => Err(Error::ExpectedInteger),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.parse_next()? {
            Parsed::Number(Number::Integer(num)) => {
                if num.is_positive() {
                    visitor.visit_u16(num as u16)
                } else {
                    Err(Error::ExpectedUnsigned)
                }
            }
            Parsed::Number(Number::Float(num)) => {
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
            _ => Err(Error::ExpectedInteger),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.parse_next()? {
            Parsed::Number(Number::Integer(num)) => {
                if num.is_positive() {
                    visitor.visit_u8(num as u8)
                } else {
                    Err(Error::ExpectedUnsigned)
                }
            }
            Parsed::Number(Number::Float(num)) => {
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
            _ => Err(Error::ExpectedInteger),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.parse_next()? {
            Parsed::Number(Number::Integer(num)) => visitor.visit_f64(num as f64),
            Parsed::Number(Number::Float(num)) => visitor.visit_f64(num),
            _ => Err(Error::ExpectedFloat),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.parse_next()? {
            Parsed::Number(Number::Integer(num)) => visitor.visit_f32(num as f32),
            Parsed::Number(Number::Float(num)) => visitor.visit_f32(num as f32),
            _ => Err(Error::ExpectedFloat),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.parse_next()? {
            Parsed::Str(string) => visitor.visit_borrowed_str(string),
            _ => Err(Error::ExpectedString),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.parse_next()? {
            Parsed::Str(string) => visitor.visit_string(string.to_string()),
            _ => Err(Error::ExpectedString),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.parse_next()? {
            Parsed::Str(string) => {
                let mut chars = string.chars();
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
            _ => Err(Error::ExpectedChar),
        }
    }

    serde::forward_to_deserialize_any! {
        bytes byte_buf option unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
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
        assert_eq!("hello", from_str::<&'static str>("'hello'")?);
        assert_eq!("hello", from_str::<String>("'hello'")?);
        assert_eq!(json!("hello"), from_str::<Value>("'hello'")?);
        Ok(())
    }

    #[test]
    fn deserialize_char() -> Result<()> {
        assert_eq!('a', from_str::<char>("'a'")?);
        assert_eq!(json!("a"), from_str::<Value>("'a'")?);
        Ok(())
    }
}
