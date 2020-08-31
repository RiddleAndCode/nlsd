use crate::format::*;
use crate::value::{Key, Map, Number, Value};
use serde::de;
use serde::de::{Error as DeError, VariantAccess};
use std::fmt;
use std::marker::PhantomData;
use time::{Date, PrimitiveDateTime, Time};

struct ValueVisitor<U, T> {
    expecting_amount: bool,
    unit: PhantomData<U>,
    custom: PhantomData<T>,
}

impl<U, T> ValueVisitor<U, T> {
    fn new() -> Self {
        Self {
            expecting_amount: false,
            unit: PhantomData,
            custom: PhantomData,
        }
    }
}

impl<'de, U, T> de::Visitor<'de> for ValueVisitor<U, T>
where
    U: std::str::FromStr + std::cmp::Ord,
    <U as std::str::FromStr>::Err: std::fmt::Display,
    T: de::Deserialize<'de>,
{
    type Value = Value<U, T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        // TODO info about the Unit and Custom type
        formatter.write_str("a valid value")
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Null)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Null)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Bool(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Number(Number::Integer(v)))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Number(Number::Integer(v as i64)))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Number(Number::Float(v)))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(
            if let Ok(datetime) = PrimitiveDateTime::parse(v, DATETIME_FORMAT) {
                Value::DateTime(datetime)
            } else if let Ok(date) = Date::parse(v, DATE_FORMAT) {
                Value::Date(date)
            } else if let Ok(time) = Time::parse(v, TIME_FORMAT) {
                Value::Time(time)
            } else if let Some(index) = v.find(' ') {
                let (num, unit) = v.split_at(index);
                if let Ok(num) = num.parse() {
                    if let Ok(unit) = unit[1..].parse() {
                        let mut map = Map::new();
                        map.insert(unit, num);
                        return Ok(Value::Amount(map));
                    }
                }
                Value::String(v.to_string())
            } else {
                Value::String(v.to_string())
            },
        )
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let string = std::str::from_utf8(v)
            .map_err(|e| E::custom(format!("only valid utf8 is supported: {}", e)))?;
        self.visit_str(string)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let mut out = Vec::new();
        while let Some(next) = seq.next_element()? {
            out.push(next);
        }
        Ok(Value::Array(out))
    }

    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        if self.expecting_amount {
            let mut out = Map::new();
            while let Some((key, value)) = map.next_entry::<String, _>()? {
                out.insert(key.parse().map_err(A::Error::custom)?, value);
            }
            self.expecting_amount = false;
            Ok(Value::Amount(out))
        } else {
            let mut out = Map::new();
            while let Some((key, value)) = map.next_entry()? {
                out.insert(key, value);
            }
            Ok(Value::Object(out))
        }
    }

    fn visit_enum<A>(mut self, data: A) -> Result<Self::Value, A::Error>
    where
        A: de::EnumAccess<'de>,
    {
        let (key, out) = data.variant::<String>()?;
        println!("enum: {}", key);
        match key.as_ref() {
            AMOUNT_VARIANT_NAME => {
                self.expecting_amount = true;
                out.struct_variant(&[], self)
            }
            CUSTOM_VARIANT_NAME => Ok(Value::Custom(out.newtype_variant::<T>()?)),
            _ => out.struct_variant(&[], self),
        }
    }
}

impl<'de, U, T> de::Deserialize<'de> for Value<U, T>
where
    U: std::str::FromStr + std::cmp::Ord,
    <U as std::str::FromStr>::Err: std::fmt::Display,
    T: de::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor::<U, T>::new())
    }
}

struct KeyVisitor;

impl<'de> de::Visitor<'de> for KeyVisitor {
    type Value = Key;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid string, bool or number key")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Key::Bool(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Key::Number(Number::Integer(v)))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Key::Number(Number::Integer(v as i64)))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Key::Number(Number::Float(v)))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Key::String(v.to_string()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Key::String(v))
    }
}

impl<'de> de::Deserialize<'de> for Key {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(KeyVisitor)
    }
}

struct NumberVisitor;

impl<'de> de::Visitor<'de> for NumberVisitor {
    type Value = Number;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid number")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Number::Integer(v))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Number::Integer(v as i64))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Number::Float(v))
    }
}

impl<'de> de::Deserialize<'de> for Number {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(NumberVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit::{NoCustom, NoUnit, SimpleValue};
    use nlsd::{from_str, Result};
    use serde::Deserialize;

    #[derive(PartialOrd, Ord, Eq, PartialEq, Debug)]
    enum Currency {
        Usd,
        Eur,
    }

    #[derive(Deserialize, Debug, PartialEq, Eq)]
    struct User {
        id: usize,
        name: String,
    }

    impl std::str::FromStr for Currency {
        type Err = &'static str;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match s {
                "dollars" => Currency::Usd,
                "euros" => Currency::Eur,
                _ => return Err(""),
            })
        }
    }

    #[test]
    fn deserialize_unit() -> Result<()> {
        assert_eq!(Value::Null, from_str::<SimpleValue>("empty")?);
        Ok(())
    }

    #[test]
    fn deserialize_bool() -> Result<()> {
        assert_eq!(Value::Bool(true), from_str::<SimpleValue>("true")?);
        assert_eq!(Value::Bool(false), from_str::<SimpleValue>("false")?);
        Ok(())
    }

    #[test]
    fn deserialize_num() -> Result<()> {
        assert_eq!(
            Value::Number(Number::Integer(1)),
            from_str::<SimpleValue>("1")?
        );
        assert_eq!(
            Value::Number(Number::Integer(0)),
            from_str::<SimpleValue>("0")?
        );
        assert_eq!(
            Value::Number(Number::Integer(-1)),
            from_str::<SimpleValue>("-1")?
        );
        assert_eq!(
            Value::Number(Number::Float(1.1)),
            from_str::<SimpleValue>("1.1")?
        );
        Ok(())
    }

    #[test]
    fn deserialize_str() -> Result<()> {
        assert_eq!(
            Value::String("Hello, world!".to_string()),
            from_str::<SimpleValue>("`Hello, world!`")?
        );
        Ok(())
    }

    #[test]
    fn deserialize_time() -> Result<()> {
        assert_eq!(
            Value::Time(time::time!(16:17:03)),
            from_str::<SimpleValue>("`16:17:03`")?
        );
        Ok(())
    }

    #[test]
    fn deserialize_date() -> Result<()> {
        assert_eq!(
            Value::Date(time::date!(2020 - 08 - 23)),
            from_str::<SimpleValue>("`August 23 2020`")?
        );
        Ok(())
    }

    #[test]
    fn deserialize_datetime() -> Result<()> {
        assert_eq!(
            Value::DateTime(PrimitiveDateTime::new(
                time::date!(2020 - 08 - 23),
                time::time!(16:17:03)
            )),
            from_str::<SimpleValue>("`August 23 16:17:03 2020`")?
        );
        Ok(())
    }

    #[test]
    fn deserialize_amount() -> Result<()> {
        assert_eq!(
            Value::<Currency, NoCustom>::Amount(
                vec![(Currency::Usd, Number::Integer(10))]
                    .into_iter()
                    .collect()
            ),
            from_str::<Value::<Currency, NoCustom>>("`10 dollars`")?
        );
        assert_eq!(
            Value::<Currency, NoCustom>::Amount(
                vec![(Currency::Eur, Number::Float(11.5))]
                    .into_iter()
                    .collect()
            ),
            from_str::<Value::<Currency, NoCustom>>("`11.5 euros`")?
        );
        assert_eq!(
            Value::<Currency, NoCustom>::String("10 bucks".to_string()),
            from_str::<Value::<Currency, NoCustom>>("`10 bucks`")?
        );
        Ok(())
    }

    #[test]
    fn deserialize_map() -> Result<()> {
        assert_eq!(
            Value::Object(
                vec![
                    (
                        Key::String("key1".to_string()),
                        Value::Number(Number::Integer(1))
                    ),
                    (
                        Key::String("key2".to_string()),
                        Value::Number(Number::Integer(2))
                    )
                ]
                .into_iter()
                .collect()
            ),
            from_str::<SimpleValue>("the object where `key1` is 1 and `key2` is 2")?
        );
        assert_eq!(
            Value::Object(
                vec![
                    (Key::String("key1".to_string()), Value::Bool(true)),
                    (Key::Number(Number::Integer(2)), Value::Null)
                ]
                .into_iter()
                .collect()
            ),
            from_str::<SimpleValue>("the object where `key1` is true and 2 is empty")?
        );
        assert_eq!(
            Value::Object(Map::new()),
            from_str::<SimpleValue>("the empty object")?
        );
        Ok(())
    }

    #[test]
    fn deserialize_array() -> Result<()> {
        assert_eq!(
            Value::Array(vec![
                Value::Number(Number::Integer(1)),
                Value::Number(Number::Integer(2))
            ]),
            from_str::<SimpleValue>("the list where an item is 1 and another item is 2")?
        );
        assert_eq!(
            Value::Array(vec![Value::Bool(true), Value::Null].into_iter().collect()),
            from_str::<SimpleValue>("the list where an item is true and another item is nothing")?
        );
        assert_eq!(
            Value::Array(vec![]),
            from_str::<SimpleValue>("the empty list")?
        );
        Ok(())
    }

    #[test]
    fn deserialize_multi_amount() -> Result<()> {
        assert_eq!(
            Value::<Currency, NoCustom>::Amount(
                vec![
                    (Currency::Usd, Number::Integer(10)),
                    (Currency::Eur, Number::Float(11.5))
                ]
                .into_iter()
                .collect()
            ),
            from_str::<Value::<Currency, NoCustom>>(
                "the `amount` where `dollars` is 10 and `euros` is 11.5"
            )?
        );
        assert!(from_str::<Value::<Currency, NoCustom>>(
            "the `amount` where `dollars` is 10 and `bucks` is 100"
        )
        .is_err());
        Ok(())
    }

    #[test]
    fn deserialize_custom() -> Result<()> {
        assert_eq!(
            Value::<NoUnit, User>::Custom(User { id: 1, name: "bob".to_string() }),
            from_str::<Value<NoUnit, User>>(
                "the `non standard object` which is the `user` where the `id` is 1 and the `name` is `bob`"
            )?
        );
        Ok(())
    }
}
