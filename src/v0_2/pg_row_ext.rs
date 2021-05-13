use ::serde::de::DeserializeOwned;
use ::tokio_postgres::Row as PgRow;

use crate::v0_2::de::Row;
use crate::v0_2::Error;

pub trait PgRowExt {
    fn cast<T>(&self) -> Result<T, Error>
    where
        T: DeserializeOwned;
}

impl PgRowExt for PgRow {
    fn cast<T>(&self) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let de = Row::new(self);
        ::serde::Deserialize::deserialize(de)
    }
}

pub trait PgRowOptionExt {
    fn cast<T>(&self) -> Result<Option<T>, Error>
    where
        T: DeserializeOwned;
}

impl PgRowOptionExt for Option<&PgRow> {
    fn cast<T>(&self) -> Result<Option<T>, Error>
    where
        T: DeserializeOwned,
    {
        self.as_ref().map(|r| r.cast()).transpose()
    }
}

pub trait PgSeqExt {
    fn cast<T>(&self) -> Result<Vec<T>, Error>
    where
        T: DeserializeOwned;
}

impl PgSeqExt for Vec<PgRow> {
    fn cast<T>(&self) -> Result<Vec<T>, Error>
    where
        T: DeserializeOwned,
    {
        self.iter().map(|r| r.cast()).collect()
    }
}
