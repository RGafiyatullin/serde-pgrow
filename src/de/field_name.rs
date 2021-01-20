use ::serde::de::Visitor;
use ::serde::Deserializer;

use crate::Error;

#[derive(Debug)]
pub struct FieldName<'de> {
    input: &'de str,
}

impl<'de> FieldName<'de> {
    pub fn new(input: &'de str) -> Self {
        Self { input }
    }
}

impl<'de> Deserializer<'de> for FieldName<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.input)
    }

    ::serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit option
        seq bytes byte_buf map unit_struct newtype_struct
        tuple_struct struct tuple enum identifier ignored_any
    }
}
