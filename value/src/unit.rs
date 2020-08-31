use crate::value::Value;
use serde::de::{self, Error as DeError};
use serde::ser::{self, Error as SerError};
use std::str::FromStr;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone, Copy)]
pub struct NoUnit;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone, Copy)]
pub struct NoCustom;

pub type SimpleValue = Value<NoUnit, NoCustom>;

pub trait UnitDisplay {
    fn unit_display(&self) -> &'static str;
}

impl UnitDisplay for NoUnit {
    fn unit_display(&self) -> &'static str {
        unimplemented!("amounts should not be constructed with the NoUnit type")
    }
}

impl FromStr for NoUnit {
    type Err = &'static str;
    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Err("")
    }
}

impl<'de> de::Deserialize<'de> for NoCustom {
    fn deserialize<D>(_: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Err(D::Error::custom("no custom object present"))
    }
}

impl ser::Serialize for NoCustom {
    fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        Err(S::Error::custom("no custom object present"))
    }
}
