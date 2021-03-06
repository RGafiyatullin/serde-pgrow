use ::serde::de::DeserializeSeed;
use ::serde::de::MapAccess;

use crate::Error;

use super::FieldName;
use super::Row;

#[derive(Debug)]
pub struct MA<'a> {
    keys: std::slice::Iter<'a, &'a str>,
    values: std::slice::Iter<'a, &'a str>,
    de: Row<'a>,
}

impl<'a> MA<'a> {
    pub fn new(fields: &'a [&'a str], de: Row<'a>) -> Self {
        Self {
            keys: fields.into_iter(),
            values: fields.into_iter(),
            de,
        }
    }
}

impl<'a, 'de> MapAccess<'de> for MA<'a> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        let key_opt = self
            .keys
            .next()
            .map(|input| seed.deserialize(FieldName::new(input)))
            .transpose()?;
        Ok(key_opt)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let value = self
            .values
            .next()
            .map(|field_name| {
                let col_name = format!(
                    "{}{}",
                    self.de
                        .col_prefix()
                        .map(|p| format!("{}_", p))
                        .unwrap_or_default(),
                    field_name
                );
                seed.deserialize(Row::new_with_prefix(self.de.pg_row(), col_name))
            })
            .transpose()?
            .expect("Failed to get another field_name for next_value_seed");
        Ok(value)
    }
}
