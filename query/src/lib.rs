use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Query<'a> {
    Index { index: usize, from_last: bool },
    Key(Cow<'a, str>),
}

impl Query<'static> {
    pub fn index(index: usize) -> Self {
        Query::Index {
            index,
            from_last: false,
        }
    }

    pub fn index_from_last(index: usize) -> Self {
        Query::Index {
            index,
            from_last: true,
        }
    }

    pub fn key_owned(key: String) -> Self {
        Query::Key(Cow::Owned(key))
    }
}

impl<'a> Query<'a> {
    pub fn key(key: &'a str) -> Self {
        Query::Key(Cow::Borrowed(key))
    }

    pub fn is_from_last(&self) -> bool {
        match self {
            Query::Index { from_last, .. } => *from_last,
            _ => false,
        }
    }

    pub fn is_key(&self) -> bool {
        match self {
            Query::Key(_) => true,
            _ => false,
        }
    }

    pub fn is_index(&self) -> bool {
        match self {
            Query::Index { .. } => true,
            _ => false,
        }
    }

    pub fn as_key(&self) -> Option<&str> {
        match self {
            Query::Key(s) => Some(s.as_ref()),
            _ => None,
        }
    }
}

pub trait AccessNext {
    fn access_next<'a>(&self, query: &Query<'a>) -> Option<&Self>;
}

pub trait AccessNextMut {
    fn access_next_mut<'a>(&mut self, query: &Query<'a>) -> Option<&mut Self>;
}

pub trait Access: AccessNext {
    fn access<'a>(&self, queries: &[Query<'a>]) -> Option<&Self> {
        queries.iter().fold(Some(self), |res, query| {
            res.and_then(|res| res.access_next(query))
        })
    }
}

pub trait AccessMut: AccessNextMut {
    fn access_mut<'a>(&mut self, queries: &[Query<'a>]) -> Option<&mut Self> {
        queries.iter().fold(Some(self), |res, query| {
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    impl AccessNext for Value {
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
                    Query::Index { .. } => None,
                    Query::Key(key) => map.get(&key.to_string()),
                },
            }
        }
    }

    impl Access for Value {}

    impl AccessNextMut for Value {
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
                    Query::Index { .. } => None,
                    Query::Key(key) => map.get_mut(&key.to_string()),
                },
            }
        }
    }

    impl AccessMut for Value {}

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
