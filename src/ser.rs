use crate::error::Error;
use serde::ser;
use std::io::Write;

pub struct Serializer<W> {
    context: Vec<String>,
    writer: W,
}

pub struct Compound<'a, W> {
    serializer: &'a mut Serializer<W>,
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
}

impl<'a, W> Compound<'a, W> {
    pub fn new(serializer: &'a mut Serializer<W>) -> Self {
        Self {
            serializer,
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
        self.writer
            .write_fmt(format_args!("'{}'", v.replace('\'', r"\'")))?;
        Ok(())
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.writer.write_all(b"the list")?;
        if self.context.is_empty() {
            self.context.push("the list".to_string());
        }
        Ok(Compound::new(self))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::Unimplemented)
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

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::Unimplemented)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::Unimplemented)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Error::Unimplemented)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::Unimplemented)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::Unimplemented)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::Unimplemented)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::Unimplemented)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        Err(Error::Unimplemented)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::Unimplemented)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        Err(Error::Unimplemented)
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
        if self.index == 0 {
            self.buffer.write_all(b" where an ")?;
        } else {
            self.buffer.write_all(b" and another ")?;
        }
        self.buffer.write_all(b"item ")?;
        // TODO check if new scope and add "of 'scope'"
        self.buffer.write_all(b"is ")?;
        self.index += 1;

        let mut context = self.serializer.context.clone();
        context.push(format!("item {}", self.index));
        let mut serializer =
            Serializer::with_context(self.buffer.clone(), self.serializer.context.clone());
        value.serialize(&mut serializer)?;
        self.buffer = serializer.writer;
        Ok(())
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        // TODO check if is leaf and append "henceforth 'scope' if not"
        self.serializer.writer.write_all(&self.buffer)?;
        Ok(())
    }
}

impl<'a, W> ser::SerializeTuple for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        Err(Error::Unimplemented)
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::Unimplemented)
    }
}

impl<'a, W> ser::SerializeTupleVariant for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        Err(Error::Unimplemented)
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::Unimplemented)
    }
}

impl<'a, W> ser::SerializeTupleStruct for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        Err(Error::Unimplemented)
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::Unimplemented)
    }
}

impl<'a, W> ser::SerializeMap for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        Err(Error::Unimplemented)
    }
    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        Err(Error::Unimplemented)
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::Unimplemented)
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
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        Err(Error::Unimplemented)
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::Unimplemented)
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
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        Err(Error::Unimplemented)
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::Unimplemented)
    }
}
