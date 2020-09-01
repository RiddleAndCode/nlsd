use crate::array::Array;
use crate::key::Key;
use crate::map::Map;
use crate::number::Number;
use time::{Date, PrimitiveDateTime, Time};

#[derive(Debug, Eq, PartialEq)]
pub enum Value<U, T> {
    Null,
    Bool(bool),
    Number(Number),
    Amount(Map<U, Number>),
    String(String),
    DateTime(PrimitiveDateTime),
    Date(Date),
    Time(Time),
    Array(Array<Value<U, T>>),
    Object(Map<Key, Value<U, T>>),
    Custom(T),
}
