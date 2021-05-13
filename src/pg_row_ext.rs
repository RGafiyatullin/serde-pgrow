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
        log::trace!("PgRowExt::cast<'de, {}>", std::any::type_name::<T>());

        if self.columns().len() == 1
            && self
                .columns()
                .first()
                .into_iter()
                .all(|c| c.type_() == &PgType::JSON || c.type_() == &PgType::JSONB)
        {
            use ::serde_json::Value as JsValue;

            let js_value_opt = self
                .try_get::<_, Option<JsValue>>(0)
                .map_err(PgDeError::PgError)?;
            ::serde::Deserialize::deserialize(js_value_opt.unwrap_or(JsValue::Null))
                .map_err(PgDeError::SerdeJsonError)
        } else {
            let de = DeRow::new(self);
            ::serde::Deserialize::deserialize(de)
        }
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
