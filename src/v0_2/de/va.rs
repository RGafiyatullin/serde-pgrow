use ::serde::de::DeserializeSeed;
use ::serde::de::SeqAccess;
use ::serde::de::Visitor;

use crate::v0_2::Error;

#[derive(Debug)]
pub struct VA<T>(std::vec::IntoIter<T>);

impl<T> From<Vec<T>> for VA<T> {
    fn from(v: Vec<T>) -> Self {
        Self(v.into_iter())
    }
}

impl<'de, E> SeqAccess<'de> for VA<E>
where
    E: Into<Prim>,
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if let Some(element) = self.0.next() {
            seed.deserialize(element.into()).map(Some)
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, ::derive_more::From)]
pub enum Prim {
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U32(u32),
    F32(f32),
    Str(String),
}

impl<'de> ::serde::Deserializer<'de> for Prim {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Self::Bool(inner) => visitor.visit_bool(inner),
            Self::I8(inner) => visitor.visit_i8(inner),
            Self::I16(inner) => visitor.visit_i16(inner),
            Self::I32(inner) => visitor.visit_i32(inner),
            Self::I64(inner) => visitor.visit_i64(inner),
            Self::U32(inner) => visitor.visit_u32(inner),
            Self::F32(inner) => visitor.visit_f32(inner),
            Self::Str(inner) => visitor.visit_string(inner),
        }
    }

    ::serde::forward_to_deserialize_any! {
        bool
        i8 i16 i32 i64
        i128
        u8 u16
        u32
        u64 u128
        f32
        f64
        char str
        string
        bytes byte_buf
        option
        unit
        unit_struct
        newtype_struct
        seq
        tuple
        tuple_struct
        map
        struct
        enum
        identifier
        ignored_any
    }
}
