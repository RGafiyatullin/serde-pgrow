use super::*;

impl<'a, 'de> SeqAccess<'de> for DeSeq<'a> {
    type Error = PgDeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        unimplemented!()
    }
}
