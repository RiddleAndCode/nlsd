use crate::number::Number;

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Key {
    Bool(bool),
    Number(Number),
    String(String),
}

impl From<bool> for Key {
    fn from(b: bool) -> Self {
        Self::Bool(b)
    }
}

impl From<String> for Key {
    fn from(string: String) -> Self {
        Key::String(string)
    }
}

impl<'a> From<&'a str> for Key {
    fn from(string: &'a str) -> Self {
        string.to_string().into()
    }
}

impl From<u8> for Key {
    fn from(i: u8) -> Self {
        Key::Number(i.into())
    }
}

impl From<u16> for Key {
    fn from(i: u16) -> Self {
        Key::Number(i.into())
    }
}

impl From<u32> for Key {
    fn from(i: u32) -> Self {
        Key::Number(i.into())
    }
}

impl From<u64> for Key {
    fn from(i: u64) -> Self {
        Key::Number(i.into())
    }
}

impl From<usize> for Key {
    fn from(i: usize) -> Self {
        Key::Number(i.into())
    }
}

impl From<i8> for Key {
    fn from(i: i8) -> Self {
        Key::Number(i.into())
    }
}

impl From<i16> for Key {
    fn from(i: i16) -> Self {
        Key::Number(i.into())
    }
}

impl From<i32> for Key {
    fn from(i: i32) -> Self {
        Key::Number(i.into())
    }
}

impl From<i64> for Key {
    fn from(i: i64) -> Self {
        Key::Number(i.into())
    }
}

impl From<isize> for Key {
    fn from(i: isize) -> Self {
        Key::Number(i.into())
    }
}

impl From<f32> for Key {
    fn from(f: f32) -> Self {
        Key::Number(f.into())
    }
}

impl From<f64> for Key {
    fn from(f: f64) -> Self {
        Key::Number(f.into())
    }
}
