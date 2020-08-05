use serde::ser;
use std::{fmt, io};

#[derive(Debug)]
pub enum Error {
    Custom(String),
    Io(io::Error),
    Unimplemented,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Self::Custom(msg.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom(msg) => f.write_fmt(format_args!("custom: {}", msg)),
            Self::Io(err) => f.write_fmt(format_args!("io: {}", err)),
            Self::Unimplemented => f.write_str("UNIMPLEMENTED"),
        }
    }
}

impl std::error::Error for Error {}
