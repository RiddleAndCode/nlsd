use nl_parser::{parse_next, Parsed};
use object_query::Query;
use std::collections::VecDeque;

/// Deserializes a str into an iterator of query parts
pub struct Deserializer<'de> {
    src: &'de str,
    first: bool,
    index: usize,
}

impl<'de> Deserializer<'de> {
    /// Construct a Deserializer for a string slice
    pub fn from_str(src: &'de str) -> Self {
        Self {
            src,
            first: true,
            index: 0,
        }
    }

    /// Get the resulting query. Keep in mind this is the reverse of the iterator due to the nature
    /// of the `of` relationships
    pub fn query(&mut self) -> Vec<Query<'de>> {
        let mut out = VecDeque::new();
        while let Some(query) = self.next() {
            out.push_front(query)
        }
        out.into()
    }

    fn parse_next(&mut self) -> Option<Parsed<'de>> {
        if let Ok((_, parsed, rest)) = parse_next(self.rest()) {
            self.index += self.rest().len() - rest.len();
            Some(parsed)
        } else {
            None
        }
    }

    fn rollback(&mut self, index: usize) {
        self.index = index
    }

    /// Returns the remaining unprocessed string slice
    #[inline]
    pub fn rest(&self) -> &'de str {
        &self.src[self.index..]
    }
}

fn parse_index(string: &str) -> Option<usize> {
    if matches!(string.chars().next()?, '1'..='9') {
        if string.ends_with("th")
            || string.ends_with("st")
            || string.ends_with("rd")
            || string.ends_with("nd")
        {
            string[..string.len() - 2].parse().ok()
        } else {
            None
        }
    } else {
        Some(match string {
            "first" => 1,
            "second" => 2,
            "third" => 3,
            "fourth" => 4,
            "fifth" => 5,
            "sixth" => 6,
            "seventh" => 7,
            "eighth" => 8,
            "ninth" => 9,
            "tenth" => 10,
            "eleventh" => 11,
            "twelfth" => 12,
            _ => return None,
        })
    }
}

impl<'a> Iterator for Deserializer<'a> {
    type Item = Query<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let start_index = self.index;

        if !self.first {
            match self.parse_next() {
                Some(Parsed::Token("of")) => (),
                _ => {
                    self.rollback(start_index);
                    return None;
                }
            }
        }
        if self.parse_next() != Some(Parsed::Token("the")) {
            self.rollback(start_index);
            return None;
        }
        let identifier = if let Some(identifier) = self.parse_next() {
            identifier
        } else {
            self.rollback(start_index);
            return None;
        };

        let indentifier_index = self.index;

        match self.parse_next() {
            Some(Parsed::Token("to")) => match self.parse_next() {
                Some(Parsed::Token("last")) => {
                    if self.parse_next() == Some(Parsed::Token("item")) {
                        match identifier {
                            Parsed::Token(index) => {
                                if let Some(index) = parse_index(index) {
                                    self.first = false;
                                    Some(Query::index_from_last(index - 1))
                                } else {
                                    self.first = false;
                                    self.rollback(indentifier_index);
                                    Some(Query::key(index))
                                }
                            }
                            Parsed::Str(key) => {
                                self.first = false;
                                self.rollback(indentifier_index);
                                Some(Query::key(key))
                            }
                            _ => {
                                self.rollback(start_index);
                                None
                            }
                        }
                    } else {
                        match identifier {
                            Parsed::Token(key) => {
                                self.first = false;
                                self.rollback(indentifier_index);
                                Some(Query::key(key))
                            }
                            Parsed::Str(key) => {
                                self.first = false;
                                self.rollback(indentifier_index);
                                Some(Query::key(key))
                            }
                            Parsed::Number(_) => {
                                self.rollback(start_index);
                                None
                            }
                        }
                    }
                }
                _ => match identifier {
                    Parsed::Token(key) => {
                        self.first = false;
                        self.rollback(indentifier_index);
                        Some(Query::key(key))
                    }
                    Parsed::Str(key) => {
                        self.first = false;
                        self.rollback(indentifier_index);
                        Some(Query::key(key))
                    }
                    Parsed::Number(_) => {
                        self.rollback(start_index);
                        None
                    }
                },
            },
            Some(Parsed::Token("item")) => match identifier {
                Parsed::Token("last") => {
                    self.first = false;
                    Some(Query::index_from_last(0))
                }
                Parsed::Token(index) => {
                    if let Some(index) = parse_index(index) {
                        self.first = false;
                        Some(Query::index(index - 1))
                    } else {
                        self.first = false;
                        self.rollback(indentifier_index);
                        Some(Query::key(index))
                    }
                }
                Parsed::Str(key) => {
                    self.first = false;
                    self.rollback(indentifier_index);
                    Some(Query::key(key))
                }
                Parsed::Number(_) => {
                    self.rollback(start_index);
                    None
                }
            },
            _ => match identifier {
                Parsed::Token(key) => {
                    self.first = false;
                    self.rollback(indentifier_index);
                    return Some(Query::key(key));
                }
                Parsed::Str(key) => {
                    self.first = false;
                    self.rollback(indentifier_index);
                    return Some(Query::key(key));
                }
                Parsed::Number(_) => {
                    self.first = false;
                    self.rollback(start_index);
                    return None;
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_indexes() {
        assert_eq!(parse_index("first"), Some(1));
        assert_eq!(parse_index("second"), Some(2));
        assert_eq!(parse_index("third"), Some(3));
        assert_eq!(parse_index("fourth"), Some(4));
        assert_eq!(parse_index("fifth"), Some(5));
        assert_eq!(parse_index("sixth"), Some(6));
        assert_eq!(parse_index("seventh"), Some(7));
        assert_eq!(parse_index("eighth"), Some(8));
        assert_eq!(parse_index("ninth"), Some(9));
        assert_eq!(parse_index("tenth"), Some(10));
        assert_eq!(parse_index("eleventh"), Some(11));
        assert_eq!(parse_index("twelfth"), Some(12));

        assert_eq!(parse_index("1st"), Some(1));
        assert_eq!(parse_index("2nd"), Some(2));
        assert_eq!(parse_index("3rd"), Some(3));
        assert_eq!(parse_index("4th"), Some(4));
        assert_eq!(parse_index("5th"), Some(5));
        assert_eq!(parse_index("6th"), Some(6));
        assert_eq!(parse_index("7th"), Some(7));
        assert_eq!(parse_index("8th"), Some(8));
        assert_eq!(parse_index("9th"), Some(9));
        assert_eq!(parse_index("10th"), Some(10));

        assert_eq!(parse_index("811th"), Some(811));
        assert_eq!(parse_index("23rd"), Some(23));
        assert_eq!(parse_index("312th"), Some(312));
        assert_eq!(parse_index("82nd"), Some(82));
        assert_eq!(parse_index("915th"), Some(915));
        assert_eq!(parse_index("55th"), Some(55));
        assert_eq!(parse_index("71st"), Some(71));

        assert!(parse_index("-1st").is_none());
        assert!(parse_index("1.1st").is_none());
        assert!(parse_index("1.1st").is_none());
        assert!(parse_index("0th").is_none());
        assert!(parse_index("01st").is_none());
    }

    #[test]
    fn deserialize_index() {
        let mut deserializer = Deserializer::from_str("the first item");
        assert_eq!(deserializer.next(), Some(Query::index(0)));
        assert!(deserializer.next().is_none());

        let mut deserializer = Deserializer::from_str("the last item");
        assert_eq!(deserializer.next(), Some(Query::index_from_last(0)));
        assert!(deserializer.next().is_none());

        let mut deserializer = Deserializer::from_str("the second item");
        assert_eq!(deserializer.next(), Some(Query::index(1)));
        assert!(deserializer.next().is_none());

        let mut deserializer = Deserializer::from_str("the 42nd item");
        assert_eq!(deserializer.next(), Some(Query::index(41)));
        assert!(deserializer.next().is_none());

        let mut deserializer = Deserializer::from_str("the third to last item");
        assert_eq!(deserializer.next(), Some(Query::index_from_last(2)));
        assert!(deserializer.next().is_none());

        let mut deserializer = Deserializer::from_str("the first item of the last item");
        assert_eq!(deserializer.next(), Some(Query::index(0)));
        assert_eq!(deserializer.next(), Some(Query::index_from_last(0)));
        assert!(deserializer.next().is_none());

        let mut deserializer = Deserializer::from_str("the 64th to last item of the sixth item");
        assert_eq!(deserializer.next(), Some(Query::index_from_last(63)));
        assert_eq!(deserializer.next(), Some(Query::index(5)));
        assert!(deserializer.next().is_none());

        let deserializer = Deserializer::from_str("the 1st item of the 2nd item of the 3rd item");
        assert_eq!(
            vec![Query::index(0), Query::index(1), Query::index(2),],
            deserializer.collect::<Vec<_>>()
        );
    }

    #[test]
    fn deserialize_key() {
        let mut deserializer = Deserializer::from_str("the key");
        assert_eq!(deserializer.next(), Some(Query::key("key")));
        assert!(deserializer.next().is_none());

        let mut deserializer = Deserializer::from_str("the `key`");
        assert_eq!(deserializer.next(), Some(Query::key("key")));
        assert!(deserializer.next().is_none());

        let mut deserializer = Deserializer::from_str("the `multi word key`");
        assert_eq!(deserializer.next(), Some(Query::key("multi word key")));
        assert!(deserializer.next().is_none());

        let mut deserializer = Deserializer::from_str("the key of the `other key`");
        assert_eq!(deserializer.next(), Some(Query::key("key")));
        assert_eq!(deserializer.next(), Some(Query::key("other key")));
        assert!(deserializer.next().is_none());

        let mut deserializer = Deserializer::from_str("the name of the user of the `access event`");
        assert_eq!(deserializer.next(), Some(Query::key("name")));
        assert_eq!(deserializer.next(), Some(Query::key("user")));
        assert_eq!(deserializer.next(), Some(Query::key("access event")));
        assert!(deserializer.next().is_none());
    }

    #[test]
    fn deserialize_mixed() {
        let mut deserializer = Deserializer::from_str("the key of the first item");
        assert_eq!(deserializer.next(), Some(Query::key("key")));
        assert_eq!(deserializer.next(), Some(Query::index(0)));
        assert!(deserializer.next().is_none());

        let mut deserializer = Deserializer::from_str("the first item of the key");
        assert_eq!(deserializer.next(), Some(Query::index(0)));
        assert_eq!(deserializer.next(), Some(Query::key("key")));
        assert!(deserializer.next().is_none());
    }

    #[test]
    fn query() {
        let mut deserializer =
            Deserializer::from_str("the 1st item of the 2nd item of the 3rd item");
        assert_eq!(
            vec![Query::index(2), Query::index(1), Query::index(0)],
            deserializer.query()
        );

        let mut deserializer = Deserializer::from_str("the name of the user of the `access event`");
        assert_eq!(
            vec![
                Query::key("access event"),
                Query::key("user"),
                Query::key("name"),
            ],
            deserializer.query()
        );

        let mut deserializer = Deserializer::from_str("the key of the first item");
        assert_eq!(
            vec![
                Query::Index {
                    index: 0,
                    from_last: false
                },
                Query::key("key"),
            ],
            deserializer.query()
        );
    }

    #[test]
    fn rest_str() {
        let mut deserializer = Deserializer::from_str("the last item of the list with extra words");
        assert_eq!(
            vec![Query::key("list"), Query::index_from_last(0)],
            deserializer.query()
        );
        assert_eq!("with extra words", deserializer.rest());
    }
}
