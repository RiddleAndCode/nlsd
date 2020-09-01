#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Query<'a> {
    Index { index: usize, from_last: bool },
    Key(&'a str),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::de::Deserializer;
    use crate::query::Query;
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
                            array.get(array.len() - index)
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
                            let index = array.len() - index;
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
        let query = Deserializer::from_str("the a").query();
        assert_eq!(value.access(&query), Some(&json!(1)));
        assert_eq!(value.access_mut(&query), Some(&mut json!(1)));
        let query = Deserializer::from_str("the b").query();
        assert_eq!(value.access(&query), Some(&json!(2)));
        let query = Deserializer::from_str("the c").query();
        assert_eq!(value.access(&query), None);
    }

    #[test]
    fn access_json_array() {
        let mut value = json!([1, 2, 3]);
        let query = Deserializer::from_str("the first item").query();
        assert_eq!(value.access(&query), Some(&json!(1)));
        assert_eq!(value.access_mut(&query), Some(&mut json!(1)));
        let query = Deserializer::from_str("the second item").query();
        assert_eq!(value.access(&query), Some(&json!(2)));
        let query = Deserializer::from_str("the last item").query();
        assert_eq!(value.access(&query), Some(&json!(3)));
        let query = Deserializer::from_str("the second to last item").query();
        assert_eq!(value.access(&query), Some(&json!(2)));
        let query = Deserializer::from_str("the fourth item").query();
        assert_eq!(value.access(&query), None);
    }

    #[test]
    fn access_json_nested() {
        let mut value = json!([{"a": 1}, [2, 3], {"c": 4}]);
        let query = Deserializer::from_str("the a of the first item").query();
        assert_eq!(value.access(&query), Some(&json!(1)));
        assert_eq!(value.access_mut(&query), Some(&mut json!(1)));
        let query = Deserializer::from_str("the c of the last item").query();
        assert_eq!(value.access(&query), Some(&json!(4)));
        let query = Deserializer::from_str("the last item of the second item").query();
        assert_eq!(value.access(&query), Some(&json!(3)));
        let query = Deserializer::from_str("the b of the first item").query();
        assert_eq!(value.access(&query), None);
        let query = Deserializer::from_str("the third item of the second item").query();
        assert_eq!(value.access(&query), None);
    }
}
