use super::*;

impl<'a, 'de> MapAccess<'de> for DeRowMap<'a> {
    type Error = PgDeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        let field_name_opt = self.keys.next();

        log::trace!(
            "next_key_seed(&mut self, ...) [K::Value = {}] => {:?}",
            std::any::type_name::<K::Value>(),
            field_name_opt
        );

        field_name_opt
            .map(|field_name| {
                let de = DeFieldName::new(field_name.as_str());
                seed.deserialize(de)
            })
            .transpose()
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let col_name = self
            .values
            .next()
            .expect("Failed to get another field_name for next_value_seed");

        let de = self
            .de_row
            .row
            .try_get::<_, Option<PgAny>>(col_name)
            .map_err(PgDeError::PgError)
            .map(PgAnyOpt::from)?;

        seed.deserialize(de)
    }
}
