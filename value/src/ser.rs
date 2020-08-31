use crate::format::*;
use crate::unit::UnitDisplay;
use crate::value::{Key, Number, Value};
use serde::ser::{self, SerializeMap, SerializeSeq, SerializeStructVariant};

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

impl<U, T> ser::Serialize for Value<U, T>
where
    U: UnitDisplay,
    T: ser::Serialize,
{
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
            Value::Time(t) => serializer.serialize_str(&t.format(TIME_FORMAT)),
            Value::Date(d) => serializer.serialize_str(&d.format(DATE_FORMAT)),
            Value::DateTime(d) => serializer.serialize_str(&d.format(DATETIME_FORMAT)),
            Value::Array(a) => {
                let mut seq = serializer.serialize_seq(Some(a.len()))?;
                for el in a {
                    seq.serialize_element(el)?;
                }
                seq.end()
            }
            Value::Object(obj) => {
                let mut map = serializer.serialize_map(Some(obj.len()))?;
                for (k, v) in obj {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
            Value::Amount(obj) => match obj.len() {
                1 => {
                    let (unit, value) = obj.into_iter().next().unwrap();
                    serializer.serialize_str(&format!("{} {}", value, unit.unit_display()))
                }
                len => {
                    let mut st =
                        serializer.serialize_struct_variant("", 0, AMOUNT_VARIANT_NAME, len)?;
                    for (k, v) in obj {
                        st.serialize_field(k.unit_display(), v)?;
                    }
                    st.end()
                }
            },
            Value::Custom(t) => serializer.serialize_newtype_variant("", 0, CUSTOM_VARIANT_NAME, t),
        }
    }
}

impl ser::Serialize for Number {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match self {
            Number::Integer(n) => serializer.serialize_i64(*n),
            Number::Float(f) => serializer.serialize_f64(*f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit::{NoCustom, NoUnit, SimpleValue};
    use nlsd::{to_string, Result};
    use serde::Serialize;
    use time::PrimitiveDateTime;

    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
    enum Currency {
        Usd,
        Eur,
    }

    #[derive(Serialize)]
    struct User {
        id: usize,
        name: &'static str,
    }

    impl UnitDisplay for Currency {
        fn unit_display(&self) -> &'static str {
            match self {
                Currency::Usd => "dollars",
                Currency::Eur => "euros",
            }
        }
    }

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

    #[test]
    fn serialize_time() -> Result<()> {
        assert_eq!(
            "`14:23:02`",
            to_string(&SimpleValue::Time(time::time!(14:23:02)))?
        );
        Ok(())
    }

    #[test]
    fn serialize_date() -> Result<()> {
        assert_eq!(
            "`August 23 2020`",
            to_string(&SimpleValue::Date(time::date!(2020 - 08 - 23)))?
        );
        Ok(())
    }

    #[test]
    fn serialize_datetime() -> Result<()> {
        assert_eq!(
            "`August 23 14:23:02 2020`",
            to_string(&SimpleValue::DateTime(PrimitiveDateTime::new(
                time::date!(2020 - 08 - 23),
                time::time!(14:23:02)
            )))?
        );
        Ok(())
    }

    #[test]
    fn serialize_map() -> Result<()> {
        assert_eq!(
            "the object where `key1` is `value1` and `key2` is `value2`",
            to_string(&SimpleValue::Object(
                vec![
                    (
                        Key::String("key1".to_string()),
                        SimpleValue::String("value1".to_string())
                    ),
                    (
                        Key::String("key2".to_string()),
                        SimpleValue::String("value2".to_string())
                    )
                ]
                .into_iter()
                .collect()
            ))?
        );
        Ok(())
    }

    #[test]
    fn serialize_list() -> Result<()> {
        assert_eq!(
            "the list where an item is `value1` and another item is `value2`",
            to_string(&SimpleValue::Array(vec![
                SimpleValue::String("value1".to_string()),
                SimpleValue::String("value2".to_string())
            ]))?
        );
        Ok(())
    }

    #[test]
    fn serialize_amount() -> Result<()> {
        assert_eq!(
            "`10 dollars`",
            to_string(&Value::<Currency, NoCustom>::Amount(
                vec![(Currency::Usd, Number::Integer(10))]
                    .into_iter()
                    .collect()
            ))?
        );
        assert_eq!(
            "the `amount` where the `dollars` is 10 and the `euros` is 11.5",
            to_string(&Value::<Currency, NoCustom>::Amount(
                vec![
                    (Currency::Usd, Number::Integer(10)),
                    (Currency::Eur, Number::Float(11.5))
                ]
                .into_iter()
                .collect()
            ))?
        );
        Ok(())
    }

    #[test]
    fn serialize_custom() -> Result<()> {
        assert_eq!(
            "the `non standard object` which is the `user` where the `id` is 1 and the `name` is `bob`",
            to_string(&Value::<NoUnit, User>::Custom(User { id: 1, name: "bob" }))?
        );
        Ok(())
    }
}
