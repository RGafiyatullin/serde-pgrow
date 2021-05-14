use super::*;

#[derive(Debug)]
pub struct DeRow<'a> {
    pub(super) row: &'a PgRow,
    pub(super) cols: Vec<&'a PgCol>,
    pub(super) prefix: Vec<&'static str>,
}

impl<'a> DeRow<'a> {
    pub(crate) fn new(row: &'a PgRow) -> Self {
        #[cfg(feature = "debug-logs")]
        log::trace!(
            "DeRow::new(...) [cols: {:?}]",
            row.columns()
                .into_iter()
                .map(PgCol::name)
                .collect::<Vec<_>>()
        );
        let cols = row.columns().iter().collect();

        Self {
            row,
            cols,
            prefix: Default::default(),
        }
    }

    pub(crate) fn proceed_with_prefix<'de, T>(
        &'a self,
        prefix: Vec<&'static str>,
        seed: T,
    ) -> Result<T::Value, PgDeError>
    where
        T: DeserializeSeed<'de>,
    {
        let row = self.row;

        #[cfg(feature = "debug-logs")]
        log::trace!(
            "DeRow::proceed_with_prefix<{}>(..., prefix: {:?}) [cols: {:?}, T::Value: {}]",
            std::any::type_name::<T>(),
            prefix,
            row.columns()
                .into_iter()
                .map(PgCol::name)
                .collect::<Vec<_>>(),
            std::any::type_name::<T::Value>(),
        );

        let field_name = prefix.join("_");
        if let Some(col) = row
            .columns()
            .iter()
            .find(|c| c.name() == field_name.as_str())
        {
            let de = row
                .try_get::<_, Option<PgAny>>(col.name())
                .map(PgAnyOpt::from)
                .unwrap();
            seed.deserialize(de)
        } else {
            let mut field_prefix = field_name;
            field_prefix.push('_');

            let cols = row
                .columns()
                .iter()
                .filter(|c| c.name().starts_with(field_prefix.as_str()))
                .collect();

            let de = Self { row, cols, prefix };
            seed.deserialize(de)
        }
    }

    pub(super) fn require_single_column<'de, T, V>(self) -> Result<PgAnyOpt<'a>, PgDeError>
    where
        V: Visitor<'de>,
    {
        #[cfg(feature = "debug-logs")]
        log::trace!(
            "require_single_column::<{}, {}>(self) [V::Value = {}]",
            std::any::type_name::<T>(),
            std::any::type_name::<V>(),
            std::any::type_name::<V::Value>()
        );

        if self.cols.len() == 1 {
            let col = self.cols[0];
            Ok(self
                .row
                .try_get::<_, Option<PgAny>>(col.name())
                .map(From::from)
                .map_err(PgDeError::PgError)?)
        } else {
            Err(PgDeError::Custom(
                format!(
                    "Failed to require_single_column::<{}, {}>: cols.len = {}",
                    std::any::type_name::<T>(),
                    std::any::type_name::<V::Value>(),
                    self.cols.len()
                ),
                None,
            ))
        }
    }
}
