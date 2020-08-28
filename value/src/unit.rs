use crate::value::Value;

pub struct NoUnit;

pub type SimpleValue = Value<NoUnit, ()>;

pub trait UnitDisplay {
    fn unit_display(&self) -> &'static str;
}

impl UnitDisplay for NoUnit {
    fn unit_display(&self) -> &'static str {
        ""
    }
}
