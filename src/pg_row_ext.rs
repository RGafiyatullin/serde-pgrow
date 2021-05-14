use ::serde::Deserialize;

use crate::de::DeRow;
use crate::de::PgDeError;
use crate::pg::*;

pub trait PgRowExt {
    fn cast<'de, T>(&self) -> Result<T, PgDeError>
    where
        T: Deserialize<'de>;
}

impl PgRowExt for PgRow {
    fn cast<'de, T>(&self) -> Result<T, PgDeError>
    where
        T: Deserialize<'de>,
    {
        #[cfg(feature = "debug-logs")]
        log::trace!("PgRowExt::cast<'de, {}>", std::any::type_name::<T>());

        let de = DeRow::new(self);
        ::serde::Deserialize::deserialize(de)
    }
}

pub trait PgRowOptionExt {
    fn cast<'de, T>(&self) -> Result<Option<T>, PgDeError>
    where
        T: Deserialize<'de>;
}

impl PgRowOptionExt for Option<&PgRow> {
    fn cast<'de, T>(&self) -> Result<Option<T>, PgDeError>
    where
        T: Deserialize<'de>,
    {
        self.as_ref().map(|r| r.cast()).transpose()
    }
}

pub trait PgSeqExt {
    fn cast<'de, T>(&self) -> Result<Vec<T>, PgDeError>
    where
        T: Deserialize<'de>;
}

impl PgSeqExt for Vec<PgRow> {
    fn cast<'de, T>(&self) -> Result<Vec<T>, PgDeError>
    where
        T: Deserialize<'de>,
    {
        self.iter().map(|r| r.cast()).collect()
    }
}
