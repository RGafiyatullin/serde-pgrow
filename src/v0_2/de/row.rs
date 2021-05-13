use ::serde::de::Visitor;
use ::tokio_postgres::Row as PgRow;

use super::MA;
use super::SA;
use super::VA;

use crate::v0_2::Error;

#[derive(Debug)]
pub struct Row<'a> {
    pg_row: &'a PgRow,
    col_prefix: Option<String>,
    col_names: Vec<&'a str>,
}

impl<'a> Row<'a> {
    pub fn col_name(&self) -> &str {
        self.col_prefix.as_ref().map(|s| s.as_ref()).unwrap_or("")
    }
}

impl<'a> Row<'a> {
    pub fn new(pg_row: &'a PgRow) -> Self {
        let col_names = pg_row
            .columns()
            .into_iter()
            .map(|c| c.name())
            .collect::<Vec<_>>();
        Self {
            pg_row,
            col_prefix: None,
            col_names,
        }
    }
    pub fn new_with_prefix<S: AsRef<str>>(pg_row: &'a PgRow, prefix: S) -> Self {
        let col_names = pg_row
            .columns()
            .into_iter()
            .map(|c| c.name())
            .filter(|c| c.starts_with(prefix.as_ref()))
            .collect::<Vec<_>>();
        Self {
            pg_row,
            col_prefix: Some(prefix.as_ref().to_owned()),
            col_names,
        }
    }

    pub fn pg_row(&self) -> &'a PgRow {
        self.pg_row
    }
    pub fn col_prefix(&self) -> Option<&str> {
        self.col_prefix.as_ref().map(|s| s.as_ref())
    }
}

impl<'a, 'de> ::serde::Deserializer<'de> for Row<'a> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.col_names.is_empty() {
            self.deserialize_unit(visitor)
        } else if self.col_names.len() == 1 {
            let col_name = self.col_names.first().unwrap();

            if let Ok(v) = self.pg_row.try_get::<_, bool>(col_name) {
                visitor.visit_bool(v)
            } else if let Ok(v) = self.pg_row.try_get::<_, i8>(col_name) {
                visitor.visit_i8(v)
            } else if let Ok(v) = self.pg_row.try_get::<_, i16>(col_name) {
                visitor.visit_i16(v)
            } else if let Ok(v) = self.pg_row.try_get::<_, i32>(col_name) {
                visitor.visit_i32(v)
            } else if let Ok(v) = self.pg_row.try_get::<_, i64>(col_name) {
                visitor.visit_i64(v)
            } else if let Ok(v) = self.pg_row.try_get::<_, u32>(col_name) {
                visitor.visit_u32(v)
            } else if let Ok(v) = self.pg_row.try_get::<_, f32>(col_name) {
                visitor.visit_f32(v)
            } else if let Ok(v) = self.pg_row.try_get::<_, String>(col_name) {
                visitor.visit_string(v)
            } else if let Ok(v) = self.pg_row.try_get::<_, Vec<bool>>(col_name) {
                visitor.visit_seq(VA::from(v))
            } else if let Ok(v) = self.pg_row.try_get::<_, Vec<i8>>(col_name) {
                visitor.visit_seq(VA::from(v))
            } else if let Ok(v) = self.pg_row.try_get::<_, Vec<i16>>(col_name) {
                visitor.visit_seq(VA::from(v))
            } else if let Ok(v) = self.pg_row.try_get::<_, Vec<i32>>(col_name) {
                visitor.visit_seq(VA::from(v))
            } else if let Ok(v) = self.pg_row.try_get::<_, Vec<i64>>(col_name) {
                visitor.visit_seq(VA::from(v))
            } else if let Ok(v) = self.pg_row.try_get::<_, Vec<u32>>(col_name) {
                visitor.visit_seq(VA::from(v))
            } else if let Ok(v) = self.pg_row.try_get::<_, Vec<f32>>(col_name) {
                visitor.visit_seq(VA::from(v))
            } else if let Ok(v) = self.pg_row.try_get::<_, Vec<String>>(col_name) {
                visitor.visit_seq(VA::from(v))
            } else {
                self.deserialize_option(visitor)
            }
        } else {
            self.deserialize_map(visitor)
        }
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
        // map
        // struct
        enum
        identifier
        ignored_any
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let ma = MA::new(
            &self.col_names[..],
            Self {
                pg_row: self.pg_row,
                col_names: self.col_names.to_owned(),
                col_prefix: self.col_prefix.to_owned(),
            },
        );
        visitor.visit_map(ma)
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
