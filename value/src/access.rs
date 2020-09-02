use crate::key::Key;
use crate::number::Number;
use crate::value::Value;
use object_query::{Access, AccessMut, AccessNext, AccessNextMut, Query};
use std::cmp;

impl<U, T> AccessNext for Value<U, T>
where
    U: cmp::Ord + std::str::FromStr,
{
    fn access_next<'a>(&self, query: &Query<'a>) -> Option<&Self> {
        match self {
            Value::Null => None,
            Value::Bool(_) => None,
            Value::Number(_) => None,
            Value::String(_) => None,
            Value::DateTime(_) => None,
            Value::Date(_) => None,
            Value::Time(_) => None,
            Value::Amount(amt) => match query {
                Query::Index { .. } => None,
                Query::Key(key) => {
                    if let Ok(unit) = key.parse() {
                        amt.get(&unit).map(|v| v.cast_unit().cast_custom())
                    } else {
                        None
                    }
                }
            },
            Value::Array(array) => match query {
                Query::Index { index, from_last } => {
                    if *from_last {
                        array.get(array.len() - 1 - index)
                    } else {
                        array.get(*index)
                    }
                }
                Query::Key(_) => None,
            },
            Value::Object(map) => match query {
                Query::Index { index, from_last } => {
                    if *from_last {
                        None
                    } else {
                        map.get(&Key::Number(Number::Integer(*index as i64)))
                    }
                }
                Query::Key(key) => map.get(&Key::String(key.to_string())),
            },
            Value::Custom(_) => None,
        }
    }
}

impl<U, T> Access for Value<U, T> where U: cmp::Ord + std::str::FromStr {}

impl<U, T> AccessNextMut for Value<U, T>
where
    U: cmp::Ord + std::str::FromStr,
{
    fn access_next_mut<'a>(&mut self, query: &Query<'a>) -> Option<&mut Self> {
        match self {
            Value::Null => None,
            Value::Bool(_) => None,
            Value::Number(_) => None,
            Value::String(_) => None,
            Value::DateTime(_) => None,
            Value::Date(_) => None,
            Value::Time(_) => None,
            Value::Amount(amt) => match query {
                Query::Index { .. } => None,
                Query::Key(key) => {
                    if let Ok(unit) = key.parse() {
                        amt.get_mut(&unit)
                            .map(|v| v.cast_unit_mut().cast_custom_mut())
                    } else {
                        None
                    }
                }
            },
            Value::Array(array) => match query {
                Query::Index { index, from_last } => {
                    if *from_last {
                        let index = array.len() - 1 - index;
                        array.get_mut(index)
                    } else {
                        array.get_mut(*index)
                    }
                }
                Query::Key(_) => None,
            },
            Value::Object(map) => match query {
                Query::Index { index, from_last } => {
                    if *from_last {
                        None
                    } else {
                        map.get_mut(&Key::Number(Number::Integer(*index as i64)))
                    }
                }
                Query::Key(key) => map.get_mut(&Key::String(key.to_string())),
            },
            Value::Custom(_) => None,
        }
    }
}

impl<U, T> AccessMut for Value<U, T> where U: cmp::Ord + std::str::FromStr {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::number::Number;
    use crate::simple::{NoCustom, SimpleValue};

    #[derive(PartialOrd, Ord, Eq, PartialEq, Debug)]
    enum Currency {
        Usd,
        Eur,
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
    fn access_object() {
        let mut value = SimpleValue::Object(
            vec![
                (Key::from("a"), Value::from(1)),
                (Key::from("b"), Value::from(2)),
            ]
            .into_iter()
            .collect(),
        );
        let query = nloq::from_str("the a");
        assert_eq!(value.access(&query), Some(&Value::from(1)));
        assert_eq!(value.access_mut(&query), Some(&mut Value::from(1)));
        let query = nloq::from_str("the b");
        assert_eq!(value.access(&query), Some(&Value::from(2)));
        let query = nloq::from_str("the c");
        assert_eq!(value.access(&query), None);
    }

    #[test]
    fn access_array() {
        let mut value = SimpleValue::Array(vec![Value::from(1), Value::from(2), Value::from(3)]);
        let query = nloq::from_str("the first item");
        assert_eq!(value.access(&query), Some(&Value::from(1)));
        assert_eq!(value.access_mut(&query), Some(&mut Value::from(1)));
        let query = nloq::from_str("the second item");
        assert_eq!(value.access(&query), Some(&Value::from(2)));
        let query = nloq::from_str("the last item");
        assert_eq!(value.access(&query), Some(&Value::from(3)));
        let query = nloq::from_str("the second to last item");
        assert_eq!(value.access(&query), Some(&Value::from(2)));
        let query = nloq::from_str("the fourth item");
        assert_eq!(value.access(&query), None);
    }

    #[test]
    fn access_nested() {
        let mut value = SimpleValue::Array(vec![
            Value::Object(vec![(Key::from("a"), Value::from(1))].into_iter().collect()),
            Value::Array(vec![Value::from(2), Value::from(3)]),
            Value::Object(vec![(Key::from("c"), Value::from(4))].into_iter().collect()),
        ]);
        let query = nloq::from_str("the a of the first item");
        assert_eq!(value.access(&query), Some(&Value::from(1)));
        assert_eq!(value.access_mut(&query), Some(&mut Value::from(1)));
        let query = nloq::from_str("the c of the last item");
        assert_eq!(value.access(&query), Some(&Value::from(4)));
        let query = nloq::from_str("the last item of the second item");
        assert_eq!(value.access(&query), Some(&Value::from(3)));
        let query = nloq::from_str("the b of the first item");
        assert_eq!(value.access(&query), None);
        let query = nloq::from_str("the third item of the second item");
        assert_eq!(value.access(&query), None);
    }

    #[test]
    fn access_amount() {
        let mut value = Value::<Currency, NoCustom>::Amount(
            vec![
                (Currency::Eur, Number::from(11.5)),
                (Currency::Usd, Number::from(10)),
            ]
            .into_iter()
            .collect(),
        );
        let query = nloq::from_str("the dollars");
        assert_eq!(value.access(&query), Some(&Value::from(10)));
        assert_eq!(value.access_mut(&query), Some(&mut Value::from(10)));
        let query = nloq::from_str("the euros");
        assert_eq!(value.access(&query), Some(&Value::from(11.5)));
        let query = nloq::from_str("the bucks");
        assert_eq!(value.access(&query), None);
    }
}
