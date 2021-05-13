use super::*;

impl<'a, 'de> Deserializer<'de> for PgAnyOpt<'a> {
    type Error = crate::de::PgDeError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_any::<{}>(...) [V::Value = {}]",
            std::any::type_name::<V>(),
            std::any::type_name::<V::Value>()
        );
        Err(PgDeError::Unimplemented(
            "de_col::deserialize_any",
            std::any::type_name::<V::Value>(),
        ))
    }

    ::serde::forward_to_deserialize_any! {
        // bool
        // i8 i16 i32 i64
        i128
        // u8 u16 u32 u64
        u128
        // f32 f64
        // char str
        // string
        // bytes byte_buf
        // option
        // unit
        // unit_struct
        // newtype_struct
        // seq
        // tuple
        // tuple_struct
        // map
        // struct
        // enum
        // identifier
        // ignored_any
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_bool(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );

        self.inner
            .ok_or(PgDeError::WasNull)?
            .deserialize_bool(visitor)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_i8(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );

        self.inner
            .ok_or(PgDeError::WasNull)?
            .deserialize_i8(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_i16(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );

        self.inner
            .ok_or(PgDeError::WasNull)?
            .deserialize_i16(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_i32(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );

        self.inner
            .ok_or(PgDeError::WasNull)?
            .deserialize_i32(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_i64(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );

        self.inner
            .ok_or(PgDeError::WasNull)?
            .deserialize_i64(visitor)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_u8(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );

        self.inner
            .ok_or(PgDeError::WasNull)?
            .deserialize_u8(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_u16(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );

        self.inner
            .ok_or(PgDeError::WasNull)?
            .deserialize_u16(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_u32(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );

        self.inner
            .ok_or(PgDeError::WasNull)?
            .deserialize_u32(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_u64(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );

        self.inner
            .ok_or(PgDeError::WasNull)?
            .deserialize_u64(visitor)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_f32(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );

        self.inner
            .ok_or(PgDeError::WasNull)?
            .deserialize_f32(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_f64(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );

        self.inner
            .ok_or(PgDeError::WasNull)?
            .deserialize_f64(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_string(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );

        self.inner
            .ok_or(PgDeError::WasNull)?
            .deserialize_string(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_option(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );
        if let Some(pg_any) = self.inner {
            visitor.visit_some(pg_any)
        } else {
            visitor.visit_none()
        }
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_newtype_struct(self, name: {:?}, ...) [V::Value = {}]",
            name,
            std::any::type_name::<V::Value>()
        );
        self.inner
            .ok_or(PgDeError::WasNull)?
            .deserialize_newtype_struct(name, visitor)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner
            .ok_or(PgDeError::WasNull)?
            .deserialize_seq(visitor)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner
            .ok_or(PgDeError::WasNull)?
            .deserialize_char(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner
            .ok_or(PgDeError::WasNull)?
            .deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner
            .ok_or(PgDeError::WasNull)?
            .deserialize_bytes(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner
            .ok_or(PgDeError::WasNull)?
            .deserialize_byte_buf(visitor)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner
            .ok_or(PgDeError::WasNull)?
            .deserialize_tuple(len, visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner
            .ok_or(PgDeError::WasNull)?
            .deserialize_tuple_struct(name, len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner
            .ok_or(PgDeError::WasNull)?
            .deserialize_map(visitor)
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner
            .ok_or(PgDeError::WasNull)?
            .deserialize_struct(name, fields, visitor)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner
            .ok_or(PgDeError::WasNull)?
            .deserialize_enum(name, variants, visitor)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner
            .ok_or(PgDeError::WasNull)?
            .deserialize_identifier(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner
            .ok_or(PgDeError::WasNull)?
            .deserialize_ignored_any(visitor)
    }
}
