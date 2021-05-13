use super::*;

impl<'a, 'de, I> SeqAccess<'de> for DeSeq<I>
where
    I: Iterator<Item = PgAnyOpt<'a>>,
{
    type Error = PgDeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if let Some(element) = self.elements.next() {
            Some(seed.deserialize(element)).transpose()
        } else {
            Ok(None)
        }
    }
}
