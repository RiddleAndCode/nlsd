use std::collections::HashMap;
use time::{PrimitiveDateTime, Time};

pub type Map<K, V> = HashMap<K, V>;

#[derive(Debug)]
pub enum Number {
    Integer(i64),
    Float(f64),
}

#[derive(Debug)]
pub enum Key {
    Bool(bool),
    Number(Number),
    String(String),
}

#[derive(Debug)]
pub enum Value<U> {
    Null,
    Bool(bool),
    Number(Number),
    Amount(Map<U, Number>),
    String(String),
    DateTime(PrimitiveDateTime),
    Time(Time),
    Array(Vec<Value<U>>),
    Object(Map<Key, Value<U>>),
}
