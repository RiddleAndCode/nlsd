use crate::format::*;
use crate::value::{Map, Number, Value};
use serde::de;
use std::fmt;
use std::marker::PhantomData;
use time::{Date, PrimitiveDateTime, Time};

struct ValueVisitor<U, T> {
    unit: PhantomData<U>,
    custom: PhantomData<T>,
}

impl<U, T> ValueVisitor<U, T> {
    fn new() -> Self {
        Self {
            unit: PhantomData,
            custom: PhantomData,
        }
    }
}

impl<'de, U, T> de::Visitor<'de> for ValueVisitor<U, T>
where
    U: std::str::FromStr + std::cmp::Ord,
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
}

impl<'de, U, T> de::Deserialize<'de> for Value<U, T>
where
    U: std::str::FromStr + std::cmp::Ord,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor::<U, T>::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit::SimpleValue;
    use nlsd::{from_str, Result};

    #[test]
    fn deserialize_unit() -> Result<()> {
        assert_eq!(Value::Null, from_str::<SimpleValue>("empty")?);
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
        #[derive(PartialOrd, Ord, Eq, PartialEq, Debug)]
        enum Currency {
            Usd,
            Eur,
        }
        impl std::str::FromStr for Currency {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(match s {
                    "dollars" => Currency::Usd,
                    "euros" => Currency::Eur,
                    _ => return Err(()),
                })
            }
        }
        assert_eq!(
            Value::<Currency, ()>::Amount(
                vec![(Currency::Usd, Number::Integer(10))]
                    .into_iter()
                    .collect()
            ),
            from_str::<Value::<Currency, ()>>("`10 dollars`")?
        );
        assert_eq!(
            Value::<Currency, ()>::Amount(
                vec![(Currency::Eur, Number::Float(11.5))]
                    .into_iter()
                    .collect()
            ),
            from_str::<Value::<Currency, ()>>("`11.5 euros`")?
        );
        assert_eq!(
            Value::<Currency, ()>::String("10 bucks".to_string()),
            from_str::<Value::<Currency, ()>>("`10 bucks`")?
        );
        Ok(())
    }
}
