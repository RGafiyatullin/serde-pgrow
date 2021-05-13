use super::*;

impl<'de> Deserializer<'de> for DeJsValue {
    type Error = PgDeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // log::trace!(
        //     "deserialize_any::<{}>(...) [type: {}; value: {:?}]",
        //     std::any::type_name::<V>(),
        //     std::any::type_name::<V::Value>(),
        //     self.value
        // );

        self.value
            .deserialize_any(visitor)
            .map_err(PgDeError::SerdeJsonError)
    }

    ::serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit option
        seq bytes byte_buf map unit_struct newtype_struct
        tuple_struct struct tuple enum identifier ignored_any
    }
}
