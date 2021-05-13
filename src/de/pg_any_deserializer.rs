use super::*;

impl<'a, 'de> Deserializer<'de> for PgAny<'a> {
    type Error = crate::de::PgDeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_any::<{}>(...) [V::Value = {}]",
            std::any::type_name::<V>(),
            std::any::type_name::<V::Value>()
        );

        match self.pg_type {
            PgType::BOOL => self.deserialize_bool(visitor),
            PgType::BOOL_ARRAY => self.deserialize_seq(visitor),

            PgType::INT2 => self.deserialize_i16(visitor),
            PgType::INT2_ARRAY => self.deserialize_seq(visitor),

            PgType::INT4 => self.deserialize_i32(visitor),
            PgType::INT4_ARRAY => self.deserialize_seq(visitor),

            PgType::INT8 => self.deserialize_i64(visitor),
            PgType::INT8_ARRAY => self.deserialize_seq(visitor),

            PgType::TEXT => self.deserialize_string(visitor),
            PgType::TEXT_ARRAY => self.deserialize_seq(visitor),
            PgType::VARCHAR => self.deserialize_string(visitor),
            PgType::VARCHAR_ARRAY => self.deserialize_seq(visitor),
            PgType::BPCHAR => self.deserialize_string(visitor),
            PgType::BPCHAR_ARRAY => self.deserialize_seq(visitor),

            PgType::FLOAT4 => self.deserialize_f32(visitor),
            PgType::FLOAT4_ARRAY => self.deserialize_seq(visitor),

            PgType::FLOAT8 => self.deserialize_f64(visitor),
            PgType::FLOAT8_ARRAY => self.deserialize_seq(visitor),

            unsupported => Err(PgDeError::UnsupportedType(unsupported)),
        }
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
        // seq
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

        let () = util::ensure_pg_type(&self.pg_type, &[PgType::BOOL])?;

        visitor.visit_bool(
            <bool as PgFromSql>::from_sql(&self.pg_type, self.raw_data)
                .map_err(PgDeError::cast_error)?,
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

        Err(PgDeError::Unimplemented(
            "pg_any::deserialize_i8",
            std::any::type_name::<V::Value>(),
        ))
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_i16(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );

        let value = match self.pg_type {
            PgType::INT2 => <i16 as PgFromSql>::from_sql(&self.pg_type, self.raw_data)
                .map_err(PgDeError::cast_error)?,

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

        let value = match self.pg_type {
            PgType::INT2 => <i16 as PgFromSql>::from_sql(&self.pg_type, self.raw_data)
                .map_err(PgDeError::cast_error)? as i32,
            PgType::INT4 => <i32 as PgFromSql>::from_sql(&self.pg_type, self.raw_data)
                .map_err(PgDeError::cast_error)?,

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

        let value = match self.pg_type {
            PgType::INT2 => <i16 as PgFromSql>::from_sql(&self.pg_type, self.raw_data)
                .map_err(PgDeError::cast_error)? as i64,
            PgType::INT4 => <i32 as PgFromSql>::from_sql(&self.pg_type, self.raw_data)
                .map_err(PgDeError::cast_error)? as i64,
            PgType::INT8 => <i64 as PgFromSql>::from_sql(&self.pg_type, self.raw_data)
                .map_err(PgDeError::cast_error)?,

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

        Err(PgDeError::Unimplemented(
            "pg_any::deserialize_u8",
            std::any::type_name::<V::Value>(),
        ))
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_u16(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );

        let value = match self.pg_type {
            PgType::INT2 => <i16 as PgFromSql>::from_sql(&self.pg_type, self.raw_data)
                .map_err(PgDeError::cast_error)? as u16,

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

        let value = match self.pg_type {
            PgType::INT2 => <i16 as PgFromSql>::from_sql(&self.pg_type, self.raw_data)
                .map_err(PgDeError::cast_error)? as u32,
            PgType::INT4 => <i32 as PgFromSql>::from_sql(&self.pg_type, self.raw_data)
                .map_err(PgDeError::cast_error)? as u32,

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

        let value = match self.pg_type {
            PgType::INT2 => <i16 as PgFromSql>::from_sql(&self.pg_type, self.raw_data)
                .map_err(PgDeError::cast_error)? as u64,
            PgType::INT4 => <i32 as PgFromSql>::from_sql(&self.pg_type, self.raw_data)
                .map_err(PgDeError::cast_error)? as u64,
            PgType::INT8 => <i64 as PgFromSql>::from_sql(&self.pg_type, self.raw_data)
                .map_err(PgDeError::cast_error)? as u64,

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

        let value = match self.pg_type {
            PgType::FLOAT4 => <f32 as PgFromSql>::from_sql(&self.pg_type, self.raw_data)
                .map_err(PgDeError::cast_error)?,

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

        let value = match self.pg_type {
            PgType::FLOAT4 => <f32 as PgFromSql>::from_sql(&self.pg_type, self.raw_data)
                .map_err(PgDeError::cast_error)? as f64,
            PgType::FLOAT8 => <f64 as PgFromSql>::from_sql(&self.pg_type, self.raw_data)
                .map_err(PgDeError::cast_error)?,

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
            &self.pg_type,
            &[PgType::VARCHAR, PgType::BPCHAR, PgType::TEXT],
        )?;

        visitor.visit_string(
            <String as PgFromSql>::from_sql(&self.pg_type, self.raw_data)
                .map_err(PgDeError::cast_error)?,
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
        let pg_any_opt = <Option<Self> as PgFromSql>::from_sql(&self.pg_type, self.raw_data)
            .map_err(PgDeError::cast_error)?;

        if let Some(pg_any) = pg_any_opt {
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
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let elements = <Vec<Option<PgAny>> as PgFromSql>::from_sql(&self.pg_type, self.raw_data)
            .map_err(PgDeError::cast_error)?
            .into_iter()
            .map(PgAnyOpt::from);
        let sa = DeSeq { elements };
        visitor.visit_seq(sa)
    }
}
