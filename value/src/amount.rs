use crate::number::Number;
use crate::simple::SimpleValue;
use crate::value::{Map, Value};
use std::{cmp, collections::btree_map, fmt, iter, ops};

#[derive(Eq, PartialEq)]
pub struct Amount<U> {
    inner: Map<U, SimpleValue>,
}

pub trait UnitDisplay {
    fn unit_display(&self) -> &'static str;
}

impl<U> Amount<U>
where
    U: cmp::Ord,
{
    pub fn new() -> Self {
        Self { inner: Map::new() }
    }

    pub fn insert(&mut self, key: U, value: Number) -> Option<SimpleValue> {
        self.inner.insert(key, Value::Number(value))
    }

    pub fn remove(&mut self, key: &U) -> Option<SimpleValue> {
        self.inner.remove(key)
    }

    pub fn get_mut(&mut self, key: &U) -> Option<&mut SimpleValue> {
        self.inner.get_mut(key)
    }
}

impl<U> From<Map<U, SimpleValue>> for Amount<U> {
    fn from(v: Map<U, SimpleValue>) -> Self {
        Self { inner: v }
    }
}

impl<U> Into<Map<U, SimpleValue>> for Amount<U> {
    fn into(self) -> Map<U, SimpleValue> {
        self.inner
    }
}

impl<U> ops::Deref for Amount<U> {
    type Target = Map<U, SimpleValue>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<U> fmt::Debug for Amount<U>
where
    U: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<U> iter::FromIterator<(U, Number)> for Amount<U>
where
    U: Ord,
{
    fn from_iter<T: IntoIterator<Item = (U, Number)>>(iter: T) -> Self {
        iter.into_iter()
            .map(|(u, n)| (u, Value::Number(n)))
            .collect::<Map<U, SimpleValue>>()
            .into()
    }
}

impl<U> IntoIterator for Amount<U> {
    type Item = (U, SimpleValue);
    type IntoIter = btree_map::IntoIter<U, SimpleValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a, U> IntoIterator for &'a Amount<U> {
    type Item = (&'a U, &'a SimpleValue);
    type IntoIter = btree_map::Iter<'a, U, SimpleValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<U> Default for Amount<U>
where
    U: cmp::Ord,
{
    fn default() -> Self {
        Map::default().into()
    }
}
