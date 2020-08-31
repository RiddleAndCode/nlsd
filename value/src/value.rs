use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;
use time::{Date, PrimitiveDateTime, Time};

pub type Map<K, V> = BTreeMap<K, V>;

#[derive(Debug)]
pub enum Number {
    Integer(i64),
    Float(f64),
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Number::Integer(left) => match other {
                Number::Integer(right) => left == right,
                Number::Float(right) => (*left as f64) == *right,
            },
            Number::Float(left) => match other {
                Number::Integer(right) => *left == (*right as f64),
                Number::Float(right) => left == right,
            },
        }
    }
}
impl Eq for Number {}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Number::Integer(left) => match other {
                Number::Integer(right) => left.partial_cmp(right),
                Number::Float(right) => (*left as f64).partial_cmp(right),
            },
            Number::Float(left) => match other {
                Number::Integer(right) => left.partial_cmp(&(*right as f64)),
                Number::Float(right) => left.partial_cmp(right),
            },
        }
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Integer(n) => n.fmt(f),
            Number::Float(n) => n.fmt(f),
        }
    }
}

impl core::hash::Hash for Number {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Number::Integer(n) => state.write_i64(*n),
            Number::Float(f) => state.write_u64(f.to_bits()),
        }
    }
}

impl std::str::FromStr for Number {
    type Err = std::num::ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = s.parse() {
            Ok(Number::Integer(num))
        } else {
            Ok(Number::Float(s.parse()?))
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Key {
    Bool(bool),
    Number(Number),
    String(String),
}

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
    Array(Vec<Value<U, T>>),
    Object(Map<Key, Value<U, T>>),
    Custom(T),
}
