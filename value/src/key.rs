use crate::number::Number;

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Key {
    Bool(bool),
    Number(Number),
    String(String),
}
