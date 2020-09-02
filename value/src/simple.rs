use crate::amount::UnitDisplay;
use crate::value::Value;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone, Copy)]
pub struct NoUnit;

impl UnitDisplay for NoUnit {
    fn unit_display(&self) -> &'static str {
        unimplemented!("amounts should not be constructed with the NoUnit type")
    }
}

impl std::str::FromStr for NoUnit {
    type Err = &'static str;
    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Err("")
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone, Copy)]
pub struct NoCustom;

pub type SimpleValue = Value<NoUnit, NoCustom>;

impl<T> Value<NoUnit, T> {
    pub fn cast_unit<U>(&self) -> &Value<U, T> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn cast_unit_mut<U>(&mut self) -> &mut Value<U, T> {
        unsafe { std::mem::transmute(self) }
    }
}

impl<U> Value<U, NoCustom> {
    pub fn cast_custom<T>(&self) -> &Value<U, T> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn cast_custom_mut<T>(&mut self) -> &mut Value<U, T> {
        unsafe { std::mem::transmute(self) }
    }
}
