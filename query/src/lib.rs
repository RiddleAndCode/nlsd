//! A representation of querying an object by either a key or an index. Normally an type implements
//! `AccessNext` and `AccessNextMut`. The `json` feature will implement this for the
//! `serde_json::Value` type

use std::borrow::Cow;

/// Either a key or an index query
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Query<'a> {
    /// The index query. Represents the index (starting at 0) from either the front or the back
    Index { index: usize, from_last: bool },
    /// The key query. Represents string key to query by
    Key(Cow<'a, str>),
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

#[cfg(test)]
mod tests {
    use super::*;
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
}
