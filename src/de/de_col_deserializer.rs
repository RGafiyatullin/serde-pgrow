use super::*;

impl<'a, 'de> Deserializer<'de> for DeCol<'a> {
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
        Err(PgDeError::Unimplemented)
    }

    ::serde::forward_to_deserialize_any! {
        // bool
        // i8 i16 i32 i64
        i128
        // u8 u16 u32 u64
        u128
        // f32 f64
        char str
        // string
        bytes byte_buf
        // option
        // unit
        // unit_struct
        // newtype_struct
        seq
        tuple
        tuple_struct
        map
        struct
        enum
        identifier
        ignored_any
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

        let () = util::ensure_pg_type(&self.col, &[PgType::BOOL])?;

        visitor.visit_bool(
            self.row
                .try_get(self.col.name())
                .map_err(PgDeError::PgError)?,
        )
    }

    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_i8(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );

        Err(PgDeError::Unimplemented)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_i16(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );

        let value = match self.col.type_() {
            &PgType::INT2 => self
                .row
                .try_get::<_, i16>(self.col.name())
                .map_err(PgDeError::PgError)?,

            unsupported => Err(PgDeError::UnsupportedType(unsupported.to_owned()))?,
        };

        visitor.visit_i16(value)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_i32(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );

        let value = match self.col.type_() {
            &PgType::INT2 => self
                .row
                .try_get::<_, i16>(self.col.name())
                .map_err(PgDeError::PgError)? as i32,
            &PgType::INT4 => self
                .row
                .try_get::<_, i32>(self.col.name())
                .map_err(PgDeError::PgError)?,

            unsupported => Err(PgDeError::UnsupportedType(unsupported.to_owned()))?,
        };

        visitor.visit_i32(value)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_i64(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );

        let value = match self.col.type_() {
            &PgType::INT2 => self
                .row
                .try_get::<_, i16>(self.col.name())
                .map_err(PgDeError::PgError)? as i64,
            &PgType::INT4 => self
                .row
                .try_get::<_, i32>(self.col.name())
                .map_err(PgDeError::PgError)? as i64,
            &PgType::INT8 => self
                .row
                .try_get::<_, i64>(self.col.name())
                .map_err(PgDeError::PgError)?,

            unsupported => Err(PgDeError::UnsupportedType(unsupported.to_owned()))?,
        };

        visitor.visit_i64(value)
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_u8(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );

        Err(PgDeError::Unimplemented)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_u16(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );

        let value = match self.col.type_() {
            &PgType::INT2 => self
                .row
                .try_get::<_, i16>(self.col.name())
                .map_err(PgDeError::PgError)? as u16,

            unsupported => Err(PgDeError::UnsupportedType(unsupported.to_owned()))?,
        };

        visitor.visit_u16(value)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_u32(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );

        let value = match self.col.type_() {
            &PgType::INT2 => self
                .row
                .try_get::<_, i16>(self.col.name())
                .map_err(PgDeError::PgError)? as u32,
            &PgType::INT4 => self
                .row
                .try_get::<_, i32>(self.col.name())
                .map_err(PgDeError::PgError)? as u32,

            unsupported => Err(PgDeError::UnsupportedType(unsupported.to_owned()))?,
        };

        visitor.visit_u32(value)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_u64(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );

        let value = match self.col.type_() {
            &PgType::INT2 => self
                .row
                .try_get::<_, i16>(self.col.name())
                .map_err(PgDeError::PgError)? as u64,
            &PgType::INT4 => self
                .row
                .try_get::<_, i32>(self.col.name())
                .map_err(PgDeError::PgError)? as u64,
            &PgType::INT8 => self
                .row
                .try_get::<_, i64>(self.col.name())
                .map_err(PgDeError::PgError)? as u64,

            unsupported => Err(PgDeError::UnsupportedType(unsupported.to_owned()))?,
        };

        visitor.visit_u64(value)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_f32(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );

        let value = match self.col.type_() {
            &PgType::FLOAT4 => self
                .row
                .try_get::<_, f32>(self.col.name())
                .map_err(PgDeError::PgError)?,

            unsupported => Err(PgDeError::UnsupportedType(unsupported.to_owned()))?,
        };

        visitor.visit_f32(value)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_f64(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );

        let value = match self.col.type_() {
            &PgType::FLOAT4 => self
                .row
                .try_get::<_, f32>(self.col.name())
                .map_err(PgDeError::PgError)? as f64,
            &PgType::FLOAT8 => self
                .row
                .try_get::<_, f64>(self.col.name())
                .map_err(PgDeError::PgError)?,

            unsupported => Err(PgDeError::UnsupportedType(unsupported.to_owned()))?,
        };

        visitor.visit_f64(value)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_string(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );

        let () = util::ensure_pg_type(
            &self.col,
            &[PgType::VARCHAR, PgType::CHAR, PgType::BPCHAR, PgType::TEXT],
        )?;

        visitor.visit_string(
            self.row
                .try_get(self.col.name())
                .map_err(PgDeError::PgError)?,
        )
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_option(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );
        let is_null = self
            .row
            .try_get::<_, Option<UnitPgType>>(self.col.name())
            .map_err(PgDeError::PgError)?
            .is_none();

        if is_null {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
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
        visitor.visit_newtype_struct(self)
    }
}
