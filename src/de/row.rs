use ::serde::de::Visitor;
use ::tokio_postgres::Row as PgRow;

use super::MA;
use super::SA;

use crate::Error;

#[derive(Debug)]
pub struct Row<'de> {
    pg_row: &'de PgRow,
    col_prefix: Option<String>,
}

impl<'de> Row<'de> {
    pub fn col_name(&self) -> &str {
        self.col_prefix.as_ref().map(|s| s.as_ref()).unwrap_or("")
    }
}

impl<'de> Row<'de> {
    pub fn new(pg_row: &'de PgRow) -> Self {
        Self {
            pg_row,
            col_prefix: None,
        }
    }
    pub fn new_with_prefix<S: AsRef<str>>(pg_row: &'de PgRow, prefix: S) -> Self {
        Self {
            pg_row,
            col_prefix: Some(prefix.as_ref().to_owned()),
        }
    }

    pub fn pg_row(&self) -> &'de PgRow {
        self.pg_row
    }
    pub fn col_prefix(&self) -> Option<&str> {
        self.col_prefix.as_ref().map(|s| s.as_ref())
    }
}

impl<'de> ::serde::Deserializer<'de> for Row<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Self::Error::Unimplemented)
    }

    ::serde::forward_to_deserialize_any! {
        // bool
        // i8 i16 i32 i64
        i128
        u8 u16
        // u32
        u64 u128
        // f32
        f64
        char str
        // string
        bytes byte_buf
        // option
        // unit
        unit_struct
        newtype_struct
        seq
        // tuple
        tuple_struct
        map
        // struct
        enum
        identifier
        ignored_any
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(MA::new(fields, self))
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SA::new(len, self))
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.pg_row.try_get(self.col_name())?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.pg_row.try_get(self.col_name())?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.pg_row.try_get(self.col_name())?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.pg_row.try_get(self.col_name())?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.pg_row.try_get(self.col_name())?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.pg_row.try_get(self.col_name())?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.pg_row.try_get(self.col_name())?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.pg_row.try_get(self.col_name())?)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self
            .pg_row
            .try_get::<_, Option<AnyPgType>>(self.col_name())?
            .is_some()
        {
            visitor.visit_some(self)
        } else {
            visitor.visit_none()
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }
}

struct AnyPgType;
impl<'a> tokio_postgres::types::FromSql<'a> for AnyPgType {
    fn from_sql(
        _ty: &tokio_postgres::types::Type,
        _raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        Ok(Self)
    }

    fn accepts(_ty: &tokio_postgres::types::Type) -> bool {
        true
    }
}
