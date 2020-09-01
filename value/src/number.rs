use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Number {
    Integer(i64),
    Float(f64),
}

impl Number {
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Number::Integer(i) => Some(*i),
            // TODO convert float to integer where possible
            Number::Float(_) => None,
        }
    }

    pub fn as_f64(&self) -> f64 {
        match self {
            Number::Integer(i) => *i as f64,
            Number::Float(f) => *f,
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Number::Integer(left) => match other {
                Number::Integer(right) => left == right,
                Number::Float(right) => (*left as f64) == *right,
            },
            Number::Float(left) => match other {
                Number::Integer(right) => *left == (*right as f64),
                Number::Float(right) => left == right,
            },
        }
    }
}
impl Eq for Number {}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Number::Integer(left) => match other {
                Number::Integer(right) => left.partial_cmp(right),
                Number::Float(right) => (*left as f64).partial_cmp(right),
            },
            Number::Float(left) => match other {
                Number::Integer(right) => left.partial_cmp(&(*right as f64)),
                Number::Float(right) => left.partial_cmp(right),
            },
        }
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Integer(n) => n.fmt(f),
            Number::Float(n) => n.fmt(f),
        }
    }
}

impl core::hash::Hash for Number {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Number::Integer(n) => state.write_i64(*n),
            Number::Float(f) => state.write_u64(f.to_bits()),
        }
    }
}

impl std::str::FromStr for Number {
    type Err = std::num::ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = s.parse() {
            Ok(Number::Integer(num))
        } else {
            Ok(Number::Float(s.parse()?))
        }
    }
}

impl From<u8> for Number {
    fn from(i: u8) -> Self {
        Number::Integer(i as i64)
    }
}

impl From<u16> for Number {
    fn from(i: u16) -> Self {
        Number::Integer(i as i64)
    }
}

impl From<u32> for Number {
    fn from(i: u32) -> Self {
        Number::Integer(i as i64)
    }
}

impl From<u64> for Number {
    fn from(i: u64) -> Self {
        Number::Integer(i as i64)
    }
}

impl From<usize> for Number {
    fn from(i: usize) -> Self {
        Number::Integer(i as i64)
    }
}

impl From<i8> for Number {
    fn from(i: i8) -> Self {
        Number::Integer(i as i64)
    }
}

impl From<i16> for Number {
    fn from(i: i16) -> Self {
        Number::Integer(i as i64)
    }
}

impl From<i32> for Number {
    fn from(i: i32) -> Self {
        Number::Integer(i as i64)
    }
}

impl From<i64> for Number {
    fn from(i: i64) -> Self {
        Number::Integer(i)
    }
}

impl From<isize> for Number {
    fn from(i: isize) -> Self {
        Number::Integer(i as i64)
    }
}

impl From<f32> for Number {
    fn from(f: f32) -> Self {
        Number::Float(f as f64)
    }
}

impl From<f64> for Number {
    fn from(f: f64) -> Self {
        Number::Float(f)
    }
}
