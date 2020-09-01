use crate::key::Key;
use crate::number::Number;
use crate::value::Value;
use object_query::{Access, AccessMut, AccessNext, AccessNextMut, Query};

impl<U, T> AccessNext for Value<U, T> {
    fn access_next<'a>(&self, query: &Query<'a>) -> Option<&Self> {
        match self {
            Value::Null => None,
            Value::Bool(_) => None,
            Value::Number(_) => None,
            Value::String(_) => None,
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
            _ => todo!("implement me"),
        }
    }
}

impl<U, T> Access for Value<U, T> {}

impl<U, T> AccessNextMut for Value<U, T> {
    fn access_next_mut<'a>(&mut self, query: &Query<'a>) -> Option<&mut Self> {
        match self {
            Value::Null => None,
            Value::Bool(_) => None,
            Value::Number(_) => None,
            Value::String(_) => None,
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
            _ => todo!("implement me"),
        }
    }
}

impl<U, T> AccessMut for Value<U, T> {}
