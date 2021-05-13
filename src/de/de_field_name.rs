use super::*;

#[derive(Debug)]
pub struct DeFieldName<'a> {
    input: &'a str,
}

impl<'a> DeFieldName<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }
}

impl<'a, 'de> Deserializer<'de> for DeFieldName<'a> {
    type Error = PgDeError;

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
