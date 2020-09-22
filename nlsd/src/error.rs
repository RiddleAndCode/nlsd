use alloc::fmt;
use alloc::string::{String, ToString};
use serde::{de, ser};

/// Error that may occur during serializing or deserializing
#[derive(Debug)]
pub enum Error {
    Custom(String),
    Fmt(fmt::Error),
    InvalidUtf8,
    Parse(nl_parser::ParseError),
    ExpectedBool,
    ExpectedNull,
    ExpectedInteger,
    ExpectedFloat,
    ExpectedUnsigned,
    ExpectedChar,
    ExpectedString,
    ExpectedKeyWord(&'static str),
    ExpectedObjectDescriptor,
    ExpectedObjectEntry,
    ExpectedListItem,
    ExpectedPrimitiveMapKey,
    ExpectedStringMapKey,
    ShouldBeDeclaredEmpty,
    ExpectedUnitVariant,
    UnexpectedUnitVariant,
}

/// Convenience wrapper for a `Result<T, Error>`
pub type Result<T, E = Error> = core::result::Result<T, E>;

impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Self {
        Self::Fmt(err)
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

impl From<nl_parser::ParseError> for Error {
    fn from(err: nl_parser::ParseError) -> Self {
        Error::Parse(err)
    }
}

impl From<core::str::Utf8Error> for Error {
    fn from(_: core::str::Utf8Error) -> Self {
        Error::InvalidUtf8
    }
}

impl de::Error for Error {
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
            Self::Fmt(err) => f.write_fmt(format_args!("io: {}", err)),
            Self::InvalidUtf8 => f.write_str("strings must be valid utf8"),
            Self::Parse(err) => f.write_fmt(format_args!("parse error: {}", err)),
            Self::ExpectedBool => f.write_str("expected boolean"),
            Self::ExpectedNull => f.write_str("expected null"),
            Self::ExpectedInteger => f.write_str("expected integer"),
            Self::ExpectedFloat => f.write_str("expected float"),
            Self::ExpectedUnsigned => f.write_str("expected unsigned"),
            Self::ExpectedChar => f.write_str("expected char"),
            Self::ExpectedString => f.write_str("expected string"),
            Self::ExpectedKeyWord(keyword) => {
                f.write_fmt(format_args!("expected keyword '{}'", keyword))
            }
            Self::ExpectedObjectDescriptor => f.write_str("expected object descriptor"),
            Self::ExpectedObjectEntry => f.write_str("expected object entry"),
            Self::ExpectedListItem => f.write_str("expected list item"),
            Self::ExpectedPrimitiveMapKey => f.write_str("map key must be primitive"),
            Self::ExpectedStringMapKey => f.write_str("expected a string map key"),
            Self::ShouldBeDeclaredEmpty => {
                f.write_str("empty objects should be declared as 'empty'")
            }
            Self::ExpectedUnitVariant => f.write_str("expected unit variant"),
            Self::UnexpectedUnitVariant => f.write_str("unexpected unit variant"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[cfg(not(feature = "std"))]
impl ser::StdError for Error {}
