use crate::amount::Amount;
use crate::key::Key;
use crate::number::Number;
use crate::unit::NoUnit;
use time::{Date, PrimitiveDateTime, Time};

pub type Map<K, V> = std::collections::BTreeMap<K, V>;

#[derive(Debug, Eq, PartialEq)]
pub enum Value<U, T> {
    Null,
    Bool(bool),
    Number(Number),
    Amount(Amount<U>),
    String(String),
    DateTime(PrimitiveDateTime),
    Date(Date),
    Time(Time),
    Array(Vec<Value<U, T>>),
    Object(Map<Key, Value<U, T>>),
    Custom(T),
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone, Copy)]
pub struct NoCustom;

pub type SimpleValue = Value<NoUnit, NoCustom>;

impl<U, T> Value<U, T> {
    pub fn is_null(&self) -> bool {
        match self {
            Value::Null => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Value::Bool(_) => true,
            _ => false,
        }
    }

    pub fn is_amount(&self) -> bool {
        match self {
            Value::Amount(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Value::String(_) => true,
            _ => false,
        }
    }

    pub fn is_date_time(&self) -> bool {
        match self {
            Value::DateTime(_) => true,
            _ => false,
        }
    }

    pub fn is_date(&self) -> bool {
        match self {
            Value::Date(_) => true,
            _ => false,
        }
    }

    pub fn is_time(&self) -> bool {
        match self {
            Value::Time(_) => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            Value::Array(_) => true,
            _ => false,
        }
    }

    pub fn is_object(&self) -> bool {
        match self {
            Value::Object(_) => true,
            _ => false,
        }
    }

    pub fn is_custom(&self) -> bool {
        match self {
            Value::Custom(_) => true,
            _ => false,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_num(&self) -> Option<&Number> {
        match self {
            Value::Number(num) => Some(num),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Number(num) => num.as_i64(),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Number(num) => Some(num.as_f64()),
            _ => None,
        }
    }

    pub fn as_amount(&self) -> Option<&Amount<U>> {
        match self {
            Value::Amount(amount) => Some(amount),
            _ => None,
        }
    }

    pub fn as_amount_mut(&mut self) -> Option<&mut Amount<U>> {
        match self {
            Value::Amount(amount) => Some(amount),
            _ => None,
        }
    }

    pub fn as_date_time(&self) -> Option<&PrimitiveDateTime> {
        match self {
            Value::DateTime(dt) => Some(dt),
            _ => None,
        }
    }

    pub fn as_date(&self) -> Option<&Date> {
        match self {
            Value::Date(d) => Some(d),
            _ => None,
        }
    }

    pub fn as_time(&self) -> Option<&Time> {
        match self {
            Value::Time(t) => Some(t),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Value<U, T>>> {
        match self {
            Value::Array(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value<U, T>>> {
        match self {
            Value::Array(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&Map<Key, Value<U, T>>> {
        match self {
            Value::Object(m) => Some(m),
            _ => None,
        }
    }

    pub fn as_object_mut(&mut self) -> Option<&mut Map<Key, Value<U, T>>> {
        match self {
            Value::Object(m) => Some(m),
            _ => None,
        }
    }

    pub fn as_custom(&self) -> Option<&T> {
        match self {
            Value::Custom(t) => Some(t),
            _ => None,
        }
    }

    pub fn as_custom_mut(&mut self) -> Option<&mut T> {
        match self {
            Value::Custom(t) => Some(t),
            _ => None,
        }
    }
}

impl<U, T> From<u8> for Value<U, T> {
    fn from(i: u8) -> Self {
        Value::Number(i.into())
    }
}

impl<U, T> From<u16> for Value<U, T> {
    fn from(i: u16) -> Self {
        Value::Number(i.into())
    }
}

impl<U, T> From<u32> for Value<U, T> {
    fn from(i: u32) -> Self {
        Value::Number(i.into())
    }
}

impl<U, T> From<u64> for Value<U, T> {
    fn from(i: u64) -> Self {
        Value::Number(i.into())
    }
}

impl<U, T> From<usize> for Value<U, T> {
    fn from(i: usize) -> Self {
        Value::Number(i.into())
    }
}

impl<U, T> From<i8> for Value<U, T> {
    fn from(i: i8) -> Self {
        Value::Number(i.into())
    }
}

impl<U, T> From<i16> for Value<U, T> {
    fn from(i: i16) -> Self {
        Value::Number(i.into())
    }
}

impl<U, T> From<i32> for Value<U, T> {
    fn from(i: i32) -> Self {
        Value::Number(i.into())
    }
}

impl<U, T> From<i64> for Value<U, T> {
    fn from(i: i64) -> Self {
        Value::Number(i.into())
    }
}

impl<U, T> From<isize> for Value<U, T> {
    fn from(i: isize) -> Self {
        Value::Number(i.into())
    }
}

impl<U, T> From<f32> for Value<U, T> {
    fn from(f: f32) -> Self {
        Value::Number(f.into())
    }
}

impl<U, T> From<f64> for Value<U, T> {
    fn from(f: f64) -> Self {
        Value::Number(f.into())
    }
}
