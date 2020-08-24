use super::error::{Error, Result};
use super::parser::{
    parse_next, parse_number, parse_string, parse_token, Number, ParseError, ParseResult, Parsed,
};
use serde::de;
use std::borrow::Cow;

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

#[derive(Debug)]
pub struct Deserializer<'de> {
    src: &'de str,
    index: usize,
    scope: Option<&'de str>,
}

fn unescape_str(string: &str) -> Cow<str> {
    let out = string.replace(r#"\`"#, "`");
    if out == string {
        Cow::Borrowed(string)
    } else {
        Cow::Owned(out)
    }
}

fn dehumanize_snake(string: &str) -> String {
    let mut out = String::new();
    let mut was_whitespace = false;
    for ch in string.chars() {
        if ch.is_whitespace() {
            was_whitespace = true;
        } else {
            if was_whitespace && !string.is_empty() {
                out.push('_');
            }
            was_whitespace = false;
            out.push(ch);
        }
    }
    out
}

fn dehumanize_match(string: &str, candidates: &[&'static str]) -> Option<&'static str> {
    if let Some(string) = candidates.into_iter().find(|&&s| s == string) {
        return Some(string);
    }
    let snake = dehumanize_snake(string);
    if let Some(string) = candidates.into_iter().find(|&&s| s == snake) {
        return Some(string);
    }
    None
}

impl<'de> Deserializer<'de> {
    pub fn from_str(src: &'de str) -> Self {
        Self {
            src,
            index: 0,
            scope: None,
        }
    }

    fn peek_next(&self) -> Result<Parsed<'de>> {
        let (_, parsed, _) =
            parse_next(&self.src[self.index..]).map_err(|err| self.inc_err_index(err.into()))?;
        Ok(parsed)
    }

    fn parse_next(&mut self) -> Result<Parsed<'de>> {
        self.inc_parse_result(parse_next(self.src()))
    }

    fn parse_token(&mut self) -> Result<&'de str> {
        self.inc_parse_result(parse_token(self.src()))
    }

    fn parse_string(&mut self) -> Result<&'de str> {
        self.inc_parse_result(parse_string(self.src()))
    }

    fn parse_number(&mut self) -> Result<Number> {
        self.inc_parse_result(parse_number(self.src()))
    }

    fn parse_and_expect_token(&mut self, token: &'static str) -> Result<()> {
        if self.parse_token()? == token {
            Ok(())
        } else {
            Err(Error::ExpectedKeyWord(token))
        }
    }

    fn inc_parse_result<T>(&mut self, result: ParseResult<T>) -> Result<T> {
        let (_, parsed, rest) = result.map_err(|err| self.inc_err_index(err.into()))?;
        self.index += self.src().len() - rest.len();
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

    #[inline]
    fn src(&self) -> &'de str {
        &self.src[self.index..]
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
                THE => {
                    let mut compound = Compound::new(self);
                    compound.describe()?;
                    if compound.is_list() {
                        visitor.visit_seq(compound)
                    } else {
                        visitor.visit_map(compound)
                    }
                }
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
                if num.is_negative() {
                    Err(Error::ExpectedUnsigned)
                } else {
                    visitor.visit_u64(num as u64)
                }
            }
            Number::Float(num) => {
                if num.trunc() == num {
                    if num.is_sign_negative() {
                        Err(Error::ExpectedUnsigned)
                    } else {
                        visitor.visit_u64(num as u64)
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
                if num.is_negative() {
                    Err(Error::ExpectedUnsigned)
                } else {
                    visitor.visit_u32(num as u32)
                }
            }
            Number::Float(num) => {
                if num.trunc() == num {
                    if num.is_sign_negative() {
                        Err(Error::ExpectedUnsigned)
                    } else {
                        visitor.visit_u32(num as u32)
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
                if num.is_negative() {
                    Err(Error::ExpectedUnsigned)
                } else {
                    visitor.visit_u16(num as u16)
                }
            }
            Number::Float(num) => {
                if num.trunc() == num {
                    if num.is_sign_negative() {
                        Err(Error::ExpectedUnsigned)
                    } else {
                        visitor.visit_u16(num as u16)
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
                if num.is_negative() {
                    Err(Error::ExpectedUnsigned)
                } else {
                    visitor.visit_u8(num as u8)
                }
            }
            Number::Float(num) => {
                if num.trunc() == num {
                    if num.is_sign_negative() {
                        Err(Error::ExpectedUnsigned)
                    } else {
                        visitor.visit_u8(num as u8)
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
        match unescape_str(self.parse_string()?) {
            Cow::Owned(string) => visitor.visit_string(string),
            Cow::Borrowed(string) => visitor.visit_borrowed_str(string),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_string(unescape_str(self.parse_string()?).into_owned())
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

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(Compound::new(self))
    }

    fn deserialize_tuple<V>(self, _: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(Compound::new(self))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _: &'static str,
        _: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(Compound::new(self))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_map(Compound::new(self))
    }

    fn deserialize_bytes<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        // TODO base64 decode?
        Err(Error::Unimplemented)
    }

    fn deserialize_byte_buf<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        // TODO base64 decode?
        Err(Error::Unimplemented)
    }

    fn deserialize_struct<V>(
        self,
        _: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_map(Compound::new_with_expected(self, fields))
    }

    serde::forward_to_deserialize_any! {
        enum ignored_any
    }
}

struct MapKey<'a, 'de> {
    de: &'a mut Deserializer<'de>,
}

struct MapExpectedKey<'a, 'de> {
    de: &'a mut Deserializer<'de>,
    expected_keys: &'static [&'static str],
}

macro_rules! forward_to_internal_de {
    ($($method:ident)*) => {
        $(
            #[inline]
            fn $method<V>(self, visitor: V) -> Result<V::Value>
            where
                V: de::Visitor<'de>,
            {
                self.de.$method(visitor)
            }
        )*
    };
}

impl<'a, 'de> de::Deserializer<'de> for MapKey<'a, 'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.de.peek_next()? {
            Parsed::Token(token) => match token {
                TRUE | FALSE | ON | OFF | ENABLED | DISABLED => self.deserialize_bool(visitor),
                EMPTY | NOTHING => self.deserialize_unit(visitor),
                _ => Err(Error::ExpectedPrimitiveMapKey),
            },
            Parsed::Number(Number::Float(_)) => self.deserialize_f64(visitor),
            Parsed::Number(Number::Integer(_)) => self.deserialize_i64(visitor),
            Parsed::Str(_) => self.deserialize_str(visitor),
        }
    }

    forward_to_internal_de!(
        deserialize_bool deserialize_i64 deserialize_i32 deserialize_i16 deserialize_i8
        deserialize_u64 deserialize_u32 deserialize_u16 deserialize_u8 deserialize_f32 deserialize_f64
        deserialize_char deserialize_str deserialize_string deserialize_unit deserialize_option
        deserialize_bytes deserialize_byte_buf
    );

    serde::forward_to_deserialize_any! {
        seq tuple tuple_struct map struct enum newtype_struct ignored_any identifier unit_struct
    }
}

impl<'a, 'de> de::Deserializer<'de> for MapExpectedKey<'a, 'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.de.peek_next()? {
            Parsed::Str(_) => self.deserialize_str(visitor),
            _ => Err(Error::ExpectedStringMapKey),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let string = self.de.parse_string()?;
        match dehumanize_match(&unescape_str(string), self.expected_keys) {
            Some(string) => visitor.visit_borrowed_str(string),
            None => visitor.visit_string(dehumanize_snake(string)),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    forward_to_internal_de! {
        deserialize_char
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

enum CompoundKind {
    List,
    Object,
}

struct Compound<'a, 'de> {
    de: &'a mut Deserializer<'de>,
    name: Option<&'de str>,
    scope: Option<&'de str>,
    kind: Option<CompoundKind>,
    is_empty: bool,
    first: bool,
    expected_keys: Option<&'static [&'static str]>,
}

impl<'a, 'de> Compound<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        Self {
            de,
            name: None,
            scope: None,
            kind: None,
            is_empty: false,
            first: true,
            expected_keys: None,
        }
    }

    fn new_with_expected(
        de: &'a mut Deserializer<'de>,
        expected_keys: &'static [&'static str],
    ) -> Self {
        Self {
            de,
            name: None,
            scope: None,
            kind: None,
            is_empty: false,
            first: true,
            expected_keys: Some(expected_keys),
        }
    }

    fn is_list(&self) -> bool {
        self.is_empty || matches!(self.kind, Some(CompoundKind::List))
    }

    fn is_object(&self) -> bool {
        self.is_empty || matches!(self.kind, Some(CompoundKind::Object))
    }

    fn is_described(&self) -> bool {
        self.kind.is_some() || self.is_empty
    }

    fn describe(&mut self) -> Result<()> {
        if self.de.parse_token()? != THE {
            return Err(Error::ExpectedKeyWord(THE));
        }
        match self.de.peek_next()? {
            Parsed::Token(EMPTY) => {
                let _ = self.de.parse_token()?;
                self.is_empty = true;
            }
            _ => (),
        }
        match self.de.parse_next()? {
            Parsed::Token(token) => match token {
                LIST => {
                    self.kind = Some(CompoundKind::List);
                }
                OBJECT => {
                    self.kind = Some(CompoundKind::Object);
                }
                _ => return Err(Error::ExpectedObjectDescriptor),
            },
            Parsed::Str(name) => self.name = Some(name),
            _ => return Err(Error::ExpectedObjectDescriptor),
        };
        match self.de.peek_next() {
            Ok(Parsed::Token(HENCEFORTH)) => {
                self.de.parse_next()?;
                match self.de.parse_next()? {
                    Parsed::Str(string) => self.scope = Some(string),
                    _ => return Err(Error::ExpectedString),
                }
            }
            _ => (),
        }
        if !self.is_empty {
            match self.de.parse_token()? {
                WHERE => (),
                _ => return Err(Error::ExpectedKeyWord(WHERE)),
            }
            if self.kind.is_none() {
                match self.de.peek_next()? {
                    Parsed::Token(token) => match token {
                        AN => self.kind = Some(CompoundKind::List),
                        THE | TRUE | FALSE | ON | OFF | ENABLED | DISABLED | EMPTY | NOTHING => {
                            self.kind = Some(CompoundKind::Object)
                        }
                        _ => return Err(Error::ExpectedKeyWord(THE)), // TODO this isnt really correct. it could be multiple tokens
                    },
                    Parsed::Str(_) => self.kind = Some(CompoundKind::Object),
                    Parsed::Number(_) => self.kind = Some(CompoundKind::Object),
                }
            }
        }
        Ok(())
    }
}

impl<'a, 'de> de::SeqAccess<'de> for Compound<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if !self.is_described() {
            self.describe()?;
        }
        if self.is_empty {
            return Ok(None);
        }
        if !self.is_list() {
            return Err(Error::ExpectedListItem);
        }
        // TODO wrap in "transaction"
        if self.first {
            self.de.parse_and_expect_token(AN)?;
        } else {
            match self.de.parse_and_expect_token(AND) {
                Ok(()) => {
                    let _ = self.de.parse_and_expect_token(ANOTHER)?;
                }
                Err(Error::Parse(ParseError::UnexpectedEof)) => return Ok(None),
                Err(err) => return Err(err),
            }
        }
        self.de.parse_and_expect_token(ITEM)?;
        // TODO check 'of `alias`'
        self.de.parse_and_expect_token(IS)?;

        let res = seed.deserialize(&mut *self.de)?;
        self.first = false;
        Ok(Some(res))
    }
}

impl<'a, 'de> de::MapAccess<'de> for Compound<'a, 'de> {
    type Error = Error;

    fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if !self.is_described() {
            self.describe()?;
        }
        if self.is_empty {
            return Ok(None);
        }
        if !self.is_object() {
            return Err(Error::ExpectedObjectEntry);
        }
        // TODO wrap in "transaction"
        if !self.first {
            match self.de.parse_and_expect_token(AND) {
                Ok(_) => (),
                Err(Error::Parse(ParseError::UnexpectedEof)) => return Ok(None),
                Err(err) => return Err(err),
            }
        }

        match self.de.peek_next()? {
            Parsed::Token(THE) => {
                let _ = self.de.parse_token()?;
            }
            _ => (),
        }

        let res = if let Some(expected_keys) = self.expected_keys {
            seed.deserialize(MapExpectedKey {
                de: &mut *self.de,
                expected_keys,
            })?
        } else {
            seed.deserialize(MapKey { de: &mut *self.de })?
        };

        self.de.parse_and_expect_token(IS)?;

        self.first = false;
        Ok(Some(res))
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::*;
    use serde::Deserialize;
    use serde_json::{json, Value};
    use std::collections::HashMap;

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

        assert_eq!(0, from_str::<u64>("0")?);
        assert_eq!(0, from_str::<u32>("0")?);
        assert_eq!(0, from_str::<u16>("0")?);
        assert_eq!(0, from_str::<u8>("0")?);
        assert_eq!(0, from_str::<i64>("0")?);
        assert_eq!(0, from_str::<i32>("0")?);
        assert_eq!(0, from_str::<i16>("0")?);
        assert_eq!(0, from_str::<i8>("0")?);
        assert_eq!(0., from_str::<f64>("0")?);
        assert_eq!(0., from_str::<f32>("0")?);

        assert_eq!(json!(1), from_str::<Value>("1")?);
        assert_eq!(json!(1.2), from_str::<Value>("1.2")?);
        assert_eq!(json!(-1), from_str::<Value>("-1")?);
        assert_eq!(json!(-1.2), from_str::<Value>("-1.2")?);

        Ok(())
    }

    #[test]
    fn deserialize_str() -> Result<()> {
        assert_eq!("hello", from_str::<String>("`hello`")?);
        assert_eq!(
            "escaped`string",
            from_str::<String>(r#"`escaped\`string`"#)?
        );

        assert_eq!("hello", from_str::<&str>("`hello`")?);
        assert!(from_str::<&str>(r#"`escaped\`string`"#).is_err());

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
        assert_eq!(Some("hello"), from_str::<Option<&str>>("`hello`")?);
        assert_eq!(Some(123), from_str::<Option<i64>>("123")?);
        assert_eq!(Some(123.123), from_str::<Option<f64>>("123.123")?);
        assert_eq!(None, from_str::<Option<i64>>("empty")?);
        assert_eq!(None, from_str::<Option<i64>>("nothing")?);
        Ok(())
    }

    #[test]
    fn deserialize_list() -> Result<()> {
        assert_eq!(Vec::<i64>::new(), from_str::<Vec<i64>>("the empty list")?);
        assert_eq!(
            Vec::<i64>::new(),
            from_str::<Vec<i64>>("the empty `named list`")?
        );
        assert_eq!(
            vec![1],
            from_str::<Vec<i64>>("the list where an item is 1")?
        );
        assert_eq!(
            vec![1],
            from_str::<Vec<i64>>("the list henceforth `aliased list` where an item is 1")?
        );
        assert_eq!(
            vec![1, 2],
            from_str::<Vec<i64>>("the list where an item is 1 and another item is 2")?
        );
        assert_eq!(
            (1, "string", true),
            from_str::<(i64, &str, bool)>(
                "the list where an item is 1 and another item is `string` and another item is true"
            )?
        );
        Ok(())
    }

    #[test]
    fn deserialize_map() -> Result<()> {
        assert_eq!(
            vec![("name", "rob")]
                .into_iter()
                .collect::<HashMap<&str, &str>>(),
            from_str::<HashMap<&str, &str>>("the object where the `name` is `rob`")?
        );
        assert_eq!(
            vec![("name", "rob"), ("id", "1")]
                .into_iter()
                .collect::<HashMap<&str, &str>>(),
            from_str::<HashMap<&str, &str>>(
                "the object where the `name` is `rob` and the `id` is `1`"
            )?
        );
        assert_eq!(
            vec![("red", 100), ("green", 200), ("blue", 50)]
                .into_iter()
                .collect::<HashMap<&str, u8>>(),
            from_str::<HashMap<&str, u8>>(
                "the object where `red` is 100 and `green` is 200 and `blue` is 50"
            )?
        );
        assert_eq!(
            vec![(false, 0), (true, 1),]
                .into_iter()
                .collect::<HashMap<bool, u8>>(),
            from_str::<HashMap<bool, u8>>("the object where true is 1 and false is 0")?
        );
        Ok(())
    }

    #[test]
    fn deserialize_struct() -> Result<()> {
        #[derive(Eq, PartialEq, Debug, Deserialize)]
        struct User {
            id: usize,
            user_name: &'static str,
        }
        assert_eq!(
            User {
                id: 1,
                user_name: "rob"
            },
            from_str::<User>("the `user` where the `user name` is `rob` and the `id` is 1")?
        );
        assert_eq!(
            User {
                id: 1,
                user_name: "rob"
            },
            from_str::<User>("the object where `user_name` is `rob` and the `id` is 1")?
        );
        Ok(())
    }
}
