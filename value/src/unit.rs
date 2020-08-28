use crate::value::Value;
use std::str::FromStr;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone, Copy)]
pub struct NoUnit;

pub type SimpleValue = Value<NoUnit, ()>;

pub trait UnitDisplay {
    fn unit_display(&self) -> &'static str;
}

impl UnitDisplay for NoUnit {
    fn unit_display(&self) -> &'static str {
        unimplemented!("amounts should not be constructed with the NoUnit type")
    }
}

impl FromStr for NoUnit {
    type Err = ();
    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Err(())
    }
}
