use super::*;

impl<'a, 'de> SeqAccess<'de> for DeRowTuple<'a> {
    type Error = PgDeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        let key_opt = self.keys.next();

        log::trace!(
            "next_element_seed(&mut self, ...) [T::Value = {}] => {:?}",
            std::any::type_name::<T::Value>(),
            key_opt
        );

        key_opt
            .map(|key| {
                let mut prefix = self.de_row.prefix.to_owned();
                let appendee = tuple_field_prefix(key, prefix.is_empty())?;
                prefix.push(appendee);
                self.de_row.proceed_with_prefix(prefix, seed)
            })
            .transpose()
    }
}

fn tuple_field_prefix(idx: usize, leading: bool) -> Result<&'static str, PgDeError> {
    let prefix = match (idx, leading) {
        (0, false) => "0",
        (1, false) => "1",
        (2, false) => "2",
        (3, false) => "3",
        (4, false) => "4",
        (5, false) => "5",
        (6, false) => "6",
        (7, false) => "7",
        (8, false) => "8",
        (9, false) => "9",
        (10, false) => "10",
        (11, false) => "11",

        (0, true) => "_0",
        (1, true) => "_1",
        (2, true) => "_2",
        (3, true) => "_3",
        (4, true) => "_4",
        (5, true) => "_5",
        (6, true) => "_6",
        (7, true) => "_7",
        (8, true) => "_8",
        (9, true) => "_9",
        (10, true) => "_10",
        (11, true) => "_11",

        _ => Err(PgDeError::Unimplemented(
            "de_row_tuple_sa::tuple_prefix",
            "arity over 12",
        ))?,
    };
    Ok(prefix)
}
