//! A representation of querying an object by either a key or an index. Normally an type implements
//! `AccessNext` and `AccessNextMut`. The `json` feature will implement this for the
//! `serde_json::Value` type
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std as core;

extern crate alloc;

use alloc::borrow::Cow;
use alloc::string::{String, ToString};
use core::iter;

/// Either a key or an index query
#[derive(Debug, PartialEq, Eq)]
pub enum Query<'a> {
    /// The index query. Represents the index (starting at 0) from either the front or the back
    Index { index: usize, from_last: bool },
    /// The key query. Represents string key to query by
    Key(Cow<'a, str>),
}

/// The result of doing a set operation
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SetResult<T> {
    /// Could not set term
    NotSet,
    /// Term was set and there was no value there before
    Set,
    /// Term replaced the value
    Replaced(T),
}

impl Query<'static> {
    /// Create an index query from the front
    pub fn index(index: usize) -> Self {
        Query::Index {
            index,
            from_last: false,
        }
    }

    /// Create an index query from the back
    pub fn index_from_last(index: usize) -> Self {
        Query::Index {
            index,
            from_last: true,
        }
    }

    /// Create a owned key query
    pub fn key_owned(key: String) -> Self {
        Query::Key(Cow::Owned(key))
    }
}

impl<'a> Query<'a> {
    /// Create a borrowed key query
    pub fn key(key: &'a str) -> Self {
        Query::Key(Cow::Borrowed(key))
    }

    /// Is an index query from the back
    pub fn is_from_last(&self) -> bool {
        match self {
            Query::Index { from_last, .. } => *from_last,
            _ => false,
        }
    }

    /// Is a key query
    pub fn is_key(&self) -> bool {
        match self {
            Query::Key(_) => true,
            _ => false,
        }
    }

    /// Is an index query
    pub fn is_index(&self) -> bool {
        match self {
            Query::Index { .. } => true,
            _ => false,
        }
    }

    /// Return the string reference if it is a key query
    pub fn as_key(&self) -> Option<&str> {
        match self {
            Query::Key(s) => Some(s.as_ref()),
            _ => None,
        }
    }

    /// Returns an i64 representation of the index if it is a index query. This is either the index
    /// or `0 - index - 1` for an index from the back
    pub fn as_index(&self) -> Option<i64> {
        match self {
            Query::Index { index, from_last } => Some(if *from_last {
                -1 - (*index as i64)
            } else {
                *index as i64
            }),
            _ => None,
        }
    }

    /// An alternative to the `std::borrow::ToOwned` method
    pub fn to_owned(&self) -> Query<'static> {
        match self {
            Query::Index { index, from_last } => Query::Index {
                index: *index,
                from_last: *from_last,
            },
            Query::Key(key) => Query::Key(Cow::Owned(key.to_string())),
        }
    }
}

impl<'a> Clone for Query<'a> {
    fn clone(&self) -> Query<'static> {
        self.to_owned()
    }
}

/// Describes how to access query
pub trait AccessNext {
    fn access_next<'a>(&self, query: &Query<'a>) -> Option<&Self>;
}

/// Describes how to access query on a mutable item
pub trait AccessNextMut {
    fn access_next_mut<'a>(&mut self, query: &Query<'a>) -> Option<&mut Self>;
}

/// An easily implementable trait to acess a list of queries
pub trait Access: AccessNext {
    fn access<'a, I: IntoIterator<Item = &'a Query<'a>>>(&self, queries: I) -> Option<&Self> {
        queries.into_iter().fold(Some(self), |res, query| {
            res.and_then(|res| res.access_next(query))
        })
    }
}

/// An easily implementable trait to acess a list of queries on a mutable item
pub trait AccessMut: AccessNextMut {
    fn access_mut<'a, I: IntoIterator<Item = &'a Query<'a>>>(
        &mut self,
        queries: I,
    ) -> Option<&mut Self> {
        queries.into_iter().fold(Some(self), |res, query| {
            res.and_then(|res| res.access_next_mut(query))
        })
    }
}

/// Describe how to set a value from a query
pub trait QuerySetItem: Sized {
    fn query_set_item<'a>(&mut self, query: &Query<'a>, val: Self) -> SetResult<Self>;
}

struct SkipLastIter<I, T>
where
    I: Iterator<Item = T>,
{
    iter: iter::Peekable<I>,
    last: Option<T>,
}

impl<I, T> SkipLastIter<I, T>
where
    I: Iterator<Item = T>,
{
    fn new(iter: I) -> Self {
        Self {
            iter: iter.peekable(),
            last: None,
        }
    }
}

impl<I, T> Iterator for SkipLastIter<I, T>
where
    I: Iterator<Item = T>,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next();
        if next.is_none() {
            return None;
        }
        if self.iter.peek().is_none() {
            self.last = next;
            return None;
        }
        next
    }
}

/// An easily implementable trait to set a value form a list of queries on a mutable item
pub trait QuerySet: QuerySetItem + AccessMut {
    fn query_set<'a, I: IntoIterator<Item = &'a Query<'a>>>(
        &mut self,
        queries: I,
        val: Self,
    ) -> SetResult<Self> {
        let mut iter = SkipLastIter::new(queries.into_iter());
        let item = self.access_mut(&mut iter);
        if let Some(item) = item {
            if let Some(last) = iter.last {
                item.query_set_item(last, val)
            } else {
                // implies empty query
                SetResult::NotSet
            }
        } else {
            // implies query not found
            SetResult::NotSet
        }
    }
}

impl From<usize> for Query<'static> {
    fn from(index: usize) -> Self {
        Self::Index {
            index,
            from_last: false,
        }
    }
}

impl From<isize> for Query<'static> {
    fn from(index: isize) -> Self {
        if index.is_negative() {
            Self::Index {
                index: index.abs() as usize - 1,
                from_last: true,
            }
        } else {
            Self::Index {
                index: index as usize,
                from_last: false,
            }
        }
    }
}

impl From<i8> for Query<'static> {
    fn from(index: i8) -> Self {
        (index as isize).into()
    }
}

impl From<i16> for Query<'static> {
    fn from(index: i16) -> Self {
        (index as isize).into()
    }
}

impl From<i32> for Query<'static> {
    fn from(index: i32) -> Self {
        (index as isize).into()
    }
}

impl From<i64> for Query<'static> {
    fn from(index: i64) -> Self {
        (index as isize).into()
    }
}

impl From<u8> for Query<'static> {
    fn from(index: u8) -> Self {
        (index as usize).into()
    }
}

impl From<u16> for Query<'static> {
    fn from(index: u16) -> Self {
        (index as usize).into()
    }
}

impl From<u32> for Query<'static> {
    fn from(index: u32) -> Self {
        (index as usize).into()
    }
}

impl From<u64> for Query<'static> {
    fn from(index: u64) -> Self {
        (index as usize).into()
    }
}

impl From<String> for Query<'static> {
    fn from(key: String) -> Self {
        Self::Key(Cow::Owned(key))
    }
}

impl<'a> From<&'a str> for Query<'a> {
    fn from(key: &'a str) -> Self {
        Self::Key(Cow::Borrowed(key))
    }
}

#[cfg(feature = "json")]
impl AccessNext for serde_json::Value {
    fn access_next<'a>(&self, query: &Query<'a>) -> Option<&Self> {
        match self {
            serde_json::Value::Null => None,
            serde_json::Value::Bool(_) => None,
            serde_json::Value::Number(_) => None,
            serde_json::Value::String(_) => None,
            serde_json::Value::Array(array) => match query {
                Query::Index { index, from_last } => {
                    if *from_last {
                        if *index >= array.len() {
                            return None;
                        }
                        array.get(array.len() - 1 - index)
                    } else {
                        array.get(*index)
                    }
                }
                Query::Key(_) => None,
            },
            serde_json::Value::Object(map) => match query {
                Query::Index { .. } => None,
                Query::Key(key) => map.get(&key.to_string()),
            },
        }
    }
}

#[cfg(feature = "json")]
impl Access for serde_json::Value {}

#[cfg(feature = "json")]
impl AccessNextMut for serde_json::Value {
    fn access_next_mut<'a>(&mut self, query: &Query<'a>) -> Option<&mut Self> {
        match self {
            serde_json::Value::Null => None,
            serde_json::Value::Bool(_) => None,
            serde_json::Value::Number(_) => None,
            serde_json::Value::String(_) => None,
            serde_json::Value::Array(array) => match query {
                Query::Index { index, from_last } => {
                    if *from_last {
                        if *index >= array.len() {
                            return None;
                        }
                        let index = array.len() - 1 - index;
                        array.get_mut(index)
                    } else {
                        array.get_mut(*index)
                    }
                }
                Query::Key(_) => None,
            },
            serde_json::Value::Object(map) => match query {
                Query::Index { .. } => None,
                Query::Key(key) => map.get_mut(&key.to_string()),
            },
        }
    }
}

#[cfg(feature = "json")]
impl AccessMut for serde_json::Value {}

#[cfg(feature = "json")]
impl QuerySetItem for serde_json::Value {
    fn query_set_item<'a>(&mut self, query: &Query<'a>, val: Self) -> SetResult<Self> {
        match self {
            serde_json::Value::Null => SetResult::NotSet,
            serde_json::Value::Bool(_) => SetResult::NotSet,
            serde_json::Value::Number(_) => SetResult::NotSet,
            serde_json::Value::String(_) => SetResult::NotSet,
            serde_json::Value::Array(array) => match query {
                Query::Index { index, from_last } => {
                    if *from_last {
                        if *index >= array.len() {
                            return SetResult::NotSet;
                        }
                        let index = array.len() - 1 - index;
                        array.push(val);
                        SetResult::Replaced(array.swap_remove(index))
                    } else {
                        if *index == array.len() {
                            array.push(val);
                            SetResult::Set
                        } else if *index > array.len() {
                            array.resize(index + 1, serde_json::Value::Null);
                            array[*index] = val;
                            SetResult::Set
                        } else {
                            array.push(val);
                            SetResult::Replaced(array.swap_remove(*index))
                        }
                    }
                }
                Query::Key(_) => SetResult::NotSet,
            },
            serde_json::Value::Object(map) => match query {
                Query::Index { .. } => SetResult::NotSet,
                Query::Key(key) => {
                    if let Some(res) = map.insert(key.to_string(), val) {
                        SetResult::Replaced(res)
                    } else {
                        SetResult::Set
                    }
                }
            },
        }
    }
}

/// Convenience macro for query arguments
///
/// ```
/// # use object_query::{query, Query};
/// assert_eq!(query!["a", 1, "b"], &[Query::key("a"), Query::index(1), Query::key("b")])
/// ```
#[macro_export]
macro_rules! query {
    ($($item:expr),*) => {
        &[$($crate::Query::from(($item))),*]
    }
}

#[cfg(feature = "json")]
impl QuerySet for serde_json::Value {}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "json")]
    use alloc::vec;
    #[cfg(feature = "json")]
    use serde_json::json;

    #[cfg(feature = "json")]
    #[test]
    fn access_json_object() {
        let mut value = json!({"a": 1, "b": 2});
        let query = vec!["a".into()];
        assert_eq!(value.access(&query), Some(&json!(1)));
        assert_eq!(value.access_mut(&query), Some(&mut json!(1)));
        let query = vec!["b".into()];
        assert_eq!(value.access(&query), Some(&json!(2)));
        let query = vec!["c".into()];
        assert_eq!(value.access(&query), None);
    }

    #[cfg(feature = "json")]
    #[test]
    fn access_json_array() {
        let mut value = json!([1, 2, 3]);
        let query = vec![0.into()];
        assert_eq!(value.access(&query), Some(&json!(1)));
        assert_eq!(value.access_mut(&query), Some(&mut json!(1)));
        let query = vec![1.into()];
        assert_eq!(value.access(&query), Some(&json!(2)));
        let query = vec![(-1).into()];
        assert_eq!(value.access(&query), Some(&json!(3)));
        let query = vec![(-2).into()];
        assert_eq!(value.access(&query), Some(&json!(2)));
        let query = vec![4.into()];
        assert_eq!(value.access(&query), None);
    }

    #[cfg(feature = "json")]
    #[test]
    fn access_json_nested() {
        let mut value = json!([{"a": 1}, [2, 3], {"c": 4}]);
        let query = vec![0.into(), "a".into()];
        assert_eq!(value.access(&query), Some(&json!(1)));
        assert_eq!(value.access_mut(&query), Some(&mut json!(1)));
        let query = vec![(-1).into(), "c".into()];
        assert_eq!(value.access(&query), Some(&json!(4)));
        let query = vec![1.into(), (-1).into()];
        assert_eq!(value.access(&query), Some(&json!(3)));
        let query = vec![0.into(), "b".into()];
        assert_eq!(value.access(&query), None);
        let query = vec![1.into(), 2.into()];
        assert_eq!(value.access(&query), None);
    }

    #[cfg(feature = "json")]
    #[test]
    fn query_set_json_array() {
        let mut value = json!([1, 2, 3]);

        let query = vec![0.into()];
        assert_eq!(
            value.query_set(&query, json!(4)),
            SetResult::Replaced(json!(1))
        );
        assert_eq!(value, json!([4, 2, 3]));

        let query = vec![3.into()];
        assert_eq!(value.query_set(&query, json!(5)), SetResult::Set);
        assert_eq!(value, json!([4, 2, 3, 5]));

        let query = vec![5.into()];
        assert_eq!(value.query_set(&query, json!(6)), SetResult::Set);
        assert_eq!(value, json!([4, 2, 3, 5, null, 6]));

        let query = vec![(-2).into()];
        assert_eq!(
            value.query_set(&query, json!(7)),
            SetResult::Replaced(json!(null))
        );
        assert_eq!(value, json!([4, 2, 3, 5, 7, 6]));

        let query = vec![(-7).into()];
        assert_eq!(value.query_set(&query, json!(9)), SetResult::NotSet);
        assert_eq!(value, json!([4, 2, 3, 5, 7, 6]));
    }

    #[cfg(feature = "json")]
    #[test]
    fn query_set_json_object() {
        let mut value = json!({"a": 1, "b": 2});

        assert_eq!(
            value.query_set(query!["a"], json!(3)),
            SetResult::Replaced(json!(1))
        );
        assert_eq!(value, json!({"a": 3, "b": 2}));

        assert_eq!(value.query_set(query!["c"], json!(4)), SetResult::Set);
        assert_eq!(value, json!({"a": 3, "b": 2, "c": 4}));
    }

    #[cfg(feature = "json")]
    #[test]
    fn query_set_json_nested() {
        let mut value = json!([{"a": 1}, [2, 3], {"c": 4}]);

        assert_eq!(
            value.query_set(query![0, "a"], json!(2)),
            SetResult::Replaced(json!(1))
        );
        assert_eq!(value, json!([{"a": 2}, [2, 3], {"c": 4}]));

        assert_eq!(value.query_set(query![2, "b"], json!(5)), SetResult::Set);
        assert_eq!(value, json!([{"a": 2}, [2, 3], {"c": 4, "b": 5}]));

        assert_eq!(value.query_set(query![1, 2], json!(6)), SetResult::Set);
        assert_eq!(value, json!([{"a": 2}, [2, 3, 6], {"c": 4, "b": 5}]));

        assert_eq!(
            value.query_set(query![-2, -3], json!(7)),
            SetResult::Replaced(json!(2))
        );
        assert_eq!(value, json!([{"a": 2}, [7, 3, 6], {"c": 4, "b": 5}]));

        assert_eq!(
            value.query_set(query![3, "key"], json!(8)),
            SetResult::NotSet
        );
        assert_eq!(value, json!([{"a": 2}, [7, 3, 6], {"c": 4, "b": 5}]));
    }
}
