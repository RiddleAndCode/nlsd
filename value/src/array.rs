use std::{fmt, iter, ops, slice, vec};

#[derive(Eq, PartialEq)]
pub struct Array<T> {
    inner: Vec<T>,
}

impl<T> Array<T> {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }
}

impl<T> From<Vec<T>> for Array<T> {
    fn from(v: Vec<T>) -> Self {
        Self { inner: v }
    }
}

impl<T> Into<Vec<T>> for Array<T> {
    fn into(self) -> Vec<T> {
        self.inner
    }
}

impl<T> ops::Deref for Array<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> fmt::Debug for Array<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> iter::FromIterator<T> for Array<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().collect::<Vec<T>>().into()
    }
}

impl<T> IntoIterator for Array<T> {
    type Item = T;
    type IntoIter = vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Array<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<T> Default for Array<T> {
    fn default() -> Self {
        Vec::default().into()
    }
}
