use ::serde::de::DeserializeSeed;
use ::serde::de::SeqAccess;

use crate::Error;

use super::Row;

#[derive(Debug)]
pub struct SA<'de> {
    iter: std::ops::Range<usize>,
    de: Row<'de>,
}

impl<'de> SA<'de> {
    pub fn new(len: usize, de: Row<'de>) -> Self {
        Self { iter: 0..len, de }
    }
}

impl<'de> SeqAccess<'de> for SA<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if let Some(idx) = self.iter.next() {
            let col_prefix = format!("{}_{}", self.de.col_prefix().unwrap_or_default(), idx);
            let de = Row::new_with_prefix(self.de.pg_row(), col_prefix);
            seed.deserialize(de).map(Some)
        } else {
            Ok(None)
        }
    }
}
