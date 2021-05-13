use super::*;

#[derive(Debug)]
pub struct DeRow<'a> {
    pub(super) row: &'a PgRow,
    pub(super) cols: Vec<&'a PgCol>,
    pub(super) prefix: Vec<&'static str>,
}

impl<'a> DeRow<'a> {
    pub(crate) fn new(row: &'a PgRow) -> Self {
        let cols = row.columns().iter().collect();

        Self {
            row,
            cols,
            prefix: Default::default(),
        }
    }
    pub(crate) fn new_with_prefix(row: &'a PgRow, prefix: Vec<&'static str>) -> Self {
        let field_name = prefix.join("_");
        if let Some(col) = row
            .columns()
            .iter()
            .find(|c| c.name() == field_name.as_str())
        {
            Self {
                row: row,
                cols: vec![col],
                prefix,
            }
        } else {
            let mut field_prefix = field_name;
            field_prefix.push('_');

            let cols = row
                .columns()
                .iter()
                .filter(|c| c.name().starts_with(field_prefix.as_str()))
                .collect();

            Self { row, cols, prefix }
        }
    }

    pub(super) fn require_single_column<'de, T, V>(self) -> Result<DeCol<'a>, PgDeError>
    where
        V: Visitor<'de>,
    {
        log::trace!(
            "require_single_column::<{}, {}>(self) [V::Value = {}]",
            std::any::type_name::<T>(),
            std::any::type_name::<V>(),
            std::any::type_name::<V::Value>()
        );

        if self.cols.len() == 1 {
            let col = self.cols[0];
            Ok(DeCol::new(self.row, col))
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
