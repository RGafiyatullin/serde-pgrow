use super::*;

impl<'a, 'de> MapAccess<'de> for DeRowStruct<'a> {
    type Error = PgDeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        let field_name_opt = self.keys.next();

        #[cfg(feature = "debug-logs")]
        log::trace!(
            "next_key_seed(&mut self, ...) [K::Value = {}] => {:?}",
            std::any::type_name::<K::Value>(),
            field_name_opt
        );

        let key_opt = field_name_opt
            .map(|input| seed.deserialize(DeFieldName::new(input)))
            .transpose()?;

        Ok(key_opt)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let field_name = self
            .values
            .next()
            .expect("Failed to get another field_name for next_value_seed");

        let mut prefix = self.de_row.prefix.to_owned();
        let () = prefix.push(field_name);

        self.de_row.proceed_with_prefix(prefix, seed)
    }
}
