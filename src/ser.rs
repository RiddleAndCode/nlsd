use crate::error::{Error, Result};
use serde::ser;
use std::io::Write;

pub struct Serializer<W> {
    context: Vec<String>,
    writer: W,
}

pub struct Compound<'a, W> {
    serializer: &'a mut Serializer<W>,
    name: Option<&'static str>,
    index: usize,
    is_leaf: bool,
    is_new_scope: bool,
    buffer: Vec<u8>,
}

impl<W> Serializer<W> {
    pub fn new(writer: W) -> Self {
        Self::with_context(writer, Default::default())
    }

    fn with_context(writer: W, context: Vec<String>) -> Self {
        Self { context, writer }
    }

    fn push_named_context(&mut self, name: &str) {
        if self.context.is_empty() {
            self.context.push(format!("the {}", humanize(name)));
        } else {
            self.context.push(humanize(name));
        }
    }

    fn push_list_context(&mut self) {
        if self.context.is_empty() {
            self.context.push("the list".to_string());
        } else {
            self.context.push("list".to_string());
        }
    }

    fn push_object_context(&mut self) {
        if self.context.is_empty() {
            self.context.push("the object".to_string());
        } else {
            self.context.push("object".to_string());
        }
    }

    fn current_scope(&self) -> String {
        self.context.join(" ")
    }

    fn parent_scope(&self) -> String {
        if self.context.is_empty() {
            String::default()
        } else {
            self.context[0..self.context.len() - 1].join(" ")
        }
    }
}

fn format_str(string: &str) -> String {
    format!("`{}`", string.replace('`', r"\`"))
}

fn humanize(string: &str) -> String {
    let mut out = String::new();
    let mut buffer = String::new();
    for ch in string.chars() {
        if ch == '_' {
            out.push(' ');
        } else if ch.is_whitespace() {
            out.push(ch);
        } else if ch.is_uppercase() {
            buffer.push(ch);
        } else if buffer.len() > 2 {
            let last = buffer.pop().unwrap();
            out.push(' ');
            out.push_str(&buffer);
            buffer.clear();
            out.push(' ');
            out.push_str(&last.to_lowercase().to_string());
            out.push(ch);
        } else if !buffer.is_empty() {
            for bch in buffer.chars() {
                out.push(' ');
                out.push_str(&bch.to_lowercase().to_string());
            }
            buffer.clear();
            out.push(ch);
        } else {
            out.push(ch)
        }
    }
    if !buffer.is_empty() {
        out.push(' ');
        out.push_str(&buffer);
    }
    out.trim().to_string()
}

impl<'a, W> Compound<'a, W> {
    pub fn new(serializer: &'a mut Serializer<W>, name: Option<&'static str>) -> Self {
        Self {
            serializer,
            name,
            index: 0,
            is_leaf: true,
            is_new_scope: false,
            buffer: Vec::default(),
        }
    }
}

impl<'a, W> ser::Serializer for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Compound<'a, W>;
    type SerializeTupleStruct = Compound<'a, W>;
    type SerializeTuple = Compound<'a, W>;
    type SerializeMap = Compound<'a, W>;
    type SerializeStruct = Compound<'a, W>;
    type SerializeStructVariant = Compound<'a, W>;
    type SerializeTupleVariant = Compound<'a, W>;

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.writer.write_fmt(format_args!("{}", v))?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.writer.write_fmt(format_args!("{}", v))?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.writer.write_fmt(format_args!("{}", v))?;
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(format_str(v).as_ref())?;
        Ok(())
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(Compound::new(self, None))
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(Compound::new(self, None))
    }

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        match v {
            true => self.writer.write_all(b"true")?,
            false => self.writer.write_all(b"false")?,
        };
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(b"nothing")?;
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(b"empty")?;
        Ok(())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::Unimplemented)
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(Compound::new(self, None))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(Compound::new(self, Some(name)))
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&humanize(name))
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&humanize(variant))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(Compound::new(self, Some(name)))
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(Compound::new(self, Some(variant)))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(Compound::new(self, Some(variant)))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        // TODO I don't know if this is right :/
        value.serialize(self)
    }
}

impl<'a, W> Compound<'a, W>
where
    W: Write,
{
    fn init_list(&mut self) {
        if self.index == 0 {
            if let Some(name) = self.name {
                self.serializer.push_named_context(name)
            } else {
                self.serializer.push_list_context()
            }
        }
    }

    fn init_object(&mut self) {
        if self.index == 0 {
            if let Some(name) = self.name {
                self.serializer.push_named_context(name)
            } else {
                self.serializer.push_object_context()
            }
        }
    }

    fn an_item(&mut self) -> Result<()> {
        if self.index == 0 {
            self.buffer.write_all(b" where an ")?;
        } else {
            self.buffer.write_all(b" and another ")?;
        }
        self.buffer.write_all(b"item ")?;
        self.index += 1;
        self.serializer.context.push(format!("item {}", self.index));
        Ok(())
    }

    fn the_key<T: ?Sized>(&mut self, name: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        if self.index == 0 {
            self.buffer.write_all(b" where ")?;
        } else {
            self.buffer.write_all(b" and ")?;
        }

        let mut serialized_name = Vec::new();
        name.serialize(&mut Serializer::new(&mut serialized_name))?;
        let name = humanize(&unsafe {
            // We do not emit invalid UTF-8.
            String::from_utf8_unchecked(serialized_name)
        });

        if !(name.starts_with('`') && name.ends_with('`') && name.len() > 1) {
            return Err(Error::UnexpectedKeyType);
        }
        let name = name[1..name.len() - 1].to_string();

        // or other verbs?
        if name.starts_with("is ") {
            self.buffer
                .write_fmt(format_args!("{} ", format_str(&name)))?;
        } else {
            self.buffer
                .write_fmt(format_args!("the {} ", format_str(&name)))?;
        }

        self.index += 1;
        self.serializer.context.push(name);
        Ok(())
    }

    fn of_scope(&mut self) -> Result<()> {
        if self.is_new_scope {
            self.buffer.write_fmt(format_args!(
                "of {} ",
                format_str(&self.serializer.parent_scope())
            ))?;
            self.is_new_scope = false;
            self.is_leaf = false;
        }
        Ok(())
    }

    fn is(&mut self) -> Result<()> {
        self.buffer.write_all(b"is ")?;
        Ok(())
    }

    fn value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        let mut serializer =
            Serializer::with_context(self.buffer.clone(), self.serializer.context.clone());
        value.serialize(&mut serializer)?;
        self.buffer = serializer.writer;
        if serializer.context.len() > self.serializer.context.len() {
            self.is_new_scope = true;
        }
        let _ = self.serializer.context.pop();
        Ok(())
    }

    fn the_list(&mut self) -> Result<()> {
        let name = if let Some(name) = self.name {
            format_str(&humanize(name))
        } else {
            "list".to_string()
        };
        if self.index > 0 {
            self.serializer
                .writer
                .write_all(format!("the {}", name).as_ref())?;
        } else {
            self.serializer
                .writer
                .write_all(format!("the empty {}", name).as_ref())?;
        }
        Ok(())
    }

    fn the_object(&mut self) -> Result<()> {
        let name = if let Some(name) = self.name {
            format_str(&humanize(name))
        } else {
            "object".to_string()
        };
        if self.index > 0 {
            self.serializer
                .writer
                .write_all(format!("the {}", name).as_ref())?;
        } else {
            self.serializer
                .writer
                .write_all(format!("the empty {}", name).as_ref())?;
        }
        Ok(())
    }

    fn contents(&mut self) -> Result<()> {
        if !self.is_leaf {
            self.serializer.writer.write_fmt(format_args!(
                " henceforth {}",
                format_str(&self.serializer.current_scope())
            ))?;
        }
        self.serializer.writer.write_all(&self.buffer)?;
        Ok(())
    }
}

impl<'a, W> ser::SerializeSeq for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        self.init_list();
        self.an_item()?;
        self.of_scope()?;
        self.is()?;
        self.value(value)?;
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.the_list()?;
        self.contents()?;
        Ok(())
    }
}

impl<'a, W> ser::SerializeTuple for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        <Self as ser::SerializeSeq>::serialize_element(self, value)
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as ser::SerializeSeq>::end(self)
    }
}

impl<'a, W> ser::SerializeTupleVariant for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        <Self as ser::SerializeSeq>::serialize_element(self, value)
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as ser::SerializeSeq>::end(self)
    }
}

impl<'a, W> ser::SerializeTupleStruct for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        <Self as ser::SerializeSeq>::serialize_element(self, value)
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as ser::SerializeSeq>::end(self)
    }
}

impl<'a, W> ser::SerializeMap for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        self.init_object();
        self.the_key(key)?;
        self.of_scope()?;
        Ok(())
    }
    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        self.is()?;
        self.value(value)?;
        Ok(())
    }
    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.the_object()?;
        self.contents()?;
        Ok(())
    }
}

impl<'a, W> ser::SerializeStruct for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        self.init_object();
        self.the_key(key)?;
        self.of_scope()?;
        self.is()?;
        self.value(value)?;
        Ok(())
    }
    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.the_object()?;
        self.contents()?;
        Ok(())
    }
}

impl<'a, W> ser::SerializeStructVariant for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        <Self as ser::SerializeStruct>::serialize_field(self, key, value)
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as ser::SerializeStruct>::end(self)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::helpers::to_string;
    use serde::Serialize;
    use std::collections::BTreeMap;

    #[test]
    fn humanize_string() {
        assert_eq!(humanize("UpperCamelCase"), "upper camel case");
        assert_eq!(humanize("camelCase"), "camel case");
        assert_eq!(humanize("snake_case"), "snake case");
        assert_eq!(humanize("CamelCaseWithACRONYM"), "camel case with ACRONYM");
        assert_eq!(humanize("middleACRONYMHere"), "middle ACRONYM here");
        assert_eq!(humanize("ACROBeginning"), "ACRO beginning");
        assert_eq!(humanize("__some_padded_name"), "some padded name");
    }

    #[test]
    fn serialize_bool() -> Result<()> {
        assert_eq!(to_string(&true)?, "true");
        assert_eq!(to_string(&false)?, "false");
        Ok(())
    }

    #[test]
    fn serialize_num() -> Result<()> {
        assert_eq!(to_string(&0)?, "0");
        assert_eq!(to_string(&1)?, "1");
        assert_eq!(to_string(&-1)?, "-1");
        assert_eq!(to_string(&12)?, "12");
        assert_eq!(to_string(&-13)?, "-13");
        assert_eq!(to_string(&1.5)?, "1.5");
        assert_eq!(to_string(&1.0)?, "1");
        assert_eq!(to_string(&0.1)?, "0.1");
        assert_eq!(to_string(&-1.5)?, "-1.5");
        Ok(())
    }

    #[test]
    fn serialize_empty() -> Result<()> {
        assert_eq!(to_string(&())?, "empty");
        assert_eq!(to_string(&Option::<()>::None)?, "nothing");
        Ok(())
    }

    #[test]
    fn serialize_str() -> Result<()> {
        assert_eq!(to_string(&'a')?, "`a`");
        assert_eq!(to_string("")?, "``");
        assert_eq!(to_string("cool")?, "`cool`");
        assert_eq!(to_string("don't")?, r"`don't`");
        assert_eq!(to_string("escaped`string")?, r"`escaped\`string`");
        Ok(())
    }

    #[test]
    fn serialize_list() -> Result<()> {
        assert_eq!(to_string(&Vec::<u8>::default())?, "the empty list");
        assert_eq!(
            to_string(&vec![1, 2, 3])?,
            "the list where an item is 1 and another item is 2 and another item is 3"
        );
        assert_eq!(to_string(&vec![vec![1, 2], vec![], vec![3, 4]])?, "the list henceforth `the list` where an item is the list where an item is 1 and another item is 2 and another item of `the list` is the empty list and another item is the list where an item is 3 and another item is 4");
        Ok(())
    }

    #[test]
    fn serialize_tuple() -> Result<()> {
        assert_eq!(to_string(&())?, "empty");
        assert_eq!(
            to_string(&(1, 2, 3))?,
            "the list where an item is 1 and another item is 2 and another item is 3"
        );
        assert_eq!(
            to_string(&(1, "string", true))?,
            "the list where an item is 1 and another item is `string` and another item is true"
        );
        assert_eq!(
        to_string(&((), (1, "cool"), (true, 4)))?,
        "the list henceforth `the list` where an item is empty and another item is the list where an item is 1 and another item is `cool` and another item of `the list` is the list where an item is true and another item is 4"
    );
        Ok(())
    }

    #[test]
    fn serialize_tuple_struct() -> Result<()> {
        #[derive(Serialize)]
        struct Example(bool, u8, String);

        #[derive(Serialize)]
        enum ExampleEnum {
            Example(bool, u8, String),
            SampleCool(String, char),
        }

        assert_eq!(
            to_string(&Example(true, 1, "cool".to_string()))?,
            "the `example` where an item is true and another item is 1 and another item is `cool`"
        );
        assert_eq!(
            to_string(&ExampleEnum::Example(true, 1, "cool".to_string()))?,
            "the `example` where an item is true and another item is 1 and another item is `cool`"
        );
        assert_eq!(
            to_string(&ExampleEnum::SampleCool("nice".to_string(), 'c'))?,
            "the `sample cool` where an item is `nice` and another item is `c`"
        );
        Ok(())
    }

    #[test]
    fn serialize_unit_struct() -> Result<()> {
        #[derive(Serialize)]
        struct Example;

        #[derive(Serialize)]
        enum ExampleEnum {
            Example,
            SampleCool,
        }

        assert_eq!(to_string(&Example)?, "`example`");
        assert_eq!(to_string(&ExampleEnum::Example)?, "`example`");
        assert_eq!(to_string(&ExampleEnum::SampleCool)?, "`sample cool`");
        Ok(())
    }

    #[test]
    fn serialize_map() -> Result<()> {
        let mut map = BTreeMap::new();
        assert_eq!(to_string(&map)?, "the empty object");
        map.insert("key", "value");
        assert_eq!(to_string(&map)?, "the object where the `key` is `value`");
        map.insert("second_key", "second value");
        assert_eq!(
            to_string(&map)?,
            "the object where the `key` is `value` and the `second key` is `second value`"
        );

        let mut map = BTreeMap::new();
        map.insert('a', 1.2);
        assert_eq!(to_string(&map)?, "the object where the `a` is 1.2");
        map.insert('b', 10.);
        assert_eq!(
            to_string(&map)?,
            "the object where the `a` is 1.2 and the `b` is 10"
        );
        Ok(())
    }

    #[test]
    fn serialize_struct() -> Result<()> {
        #[derive(Serialize)]
        struct User {
            id: i32,
            name: String,
            roles: Vec<String>,
        };

        assert_eq!(to_string(&User { id: 1, name: "user".to_string(), roles: vec!["Admin".to_string()] })?, "the `user` where the `id` is 1 and the `name` is `user` and the `roles` is the list where an item is `Admin`");
        Ok(())
    }
}
