use super::*;

impl<'a, 'de> Deserializer<'de> for DeRow<'a> {
    type Error = crate::de::PgDeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.cols.is_empty() {
            visitor.visit_unit()
        } else if self.cols.len() == 1 {
            let col = self.cols[0];
            match col.type_() {
                unsupported => Err(PgDeError::UnsupportedType(unsupported.to_owned())),
            }
        } else {
            println!("columns: {:#?}", self.cols);
            unimplemented!("deserialize_any (many columns)")
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
        // tuple
        // tuple_struct
        // map
        // struct
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

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_tuple(self, len: {:?}, ...) [V::Value = {}]",
            len,
            std::any::type_name::<V::Value>()
        );
        visitor.visit_seq(DeRowTuple::new(&self, None, len))
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
        log::trace!(
            "deserialize_tuple_struct(self, name: {:?} len: {:?}, ...) [V::Value = {}]",
            name,
            len,
            std::any::type_name::<V::Value>()
        );
        visitor.visit_seq(DeRowTuple::new(&self, Some(name), len))
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_seq(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );
        Err(PgDeError::Unimplemented)
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
        log::trace!(
            "deserialize_struct(self, name: {:?}, fields: {:?}, ...) [V::Value = {}]",
            name,
            fields,
            std::any::type_name::<V::Value>()
        );

        let ma = DeRowStruct::new(&self, name, fields);

        visitor.visit_map(ma)
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_map(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );
        Err(PgDeError::Unimplemented)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "deserialize_bool(self, ...) [V::Value = {}]",
            std::any::type_name::<V::Value>()
        );

        self.require_single_column::<bool, V>()?
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

        self.require_single_column::<i8, V>()?
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

        self.require_single_column::<i16, V>()?
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

        self.require_single_column::<i32, V>()?
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

        self.require_single_column::<i32, V>()?
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

        self.require_single_column::<u8, V>()?
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

        self.require_single_column::<u16, V>()?
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

        self.require_single_column::<u32, V>()?
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

        self.require_single_column::<i32, V>()?
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

        self.require_single_column::<f32, V>()?
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

        self.require_single_column::<f64, V>()?
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

        self.require_single_column::<String, V>()?
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

        self.require_single_column::<Option<V::Value>, V>()?
            .deserialize_option(visitor)
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
            "deserialize_newtype_struct(self, name: {:?} ...) [V::Value = {}]",
            name,
            std::any::type_name::<V::Value>()
        );

        self.require_single_column::<Option<V::Value>, V>()?
            .deserialize_newtype_struct(name, visitor)
    }
}
