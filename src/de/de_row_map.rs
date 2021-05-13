use super::*;

#[derive(Debug)]
pub struct DeRowMap<'a> {
    pub(super) de_row: &'a DeRow<'a>,
    pub(super) keys: std::vec::IntoIter<String>,
    pub(super) values: std::vec::IntoIter<&'a str>,
}

impl<'a> DeRowMap<'a> {
    pub(super) fn new(de_row: &'a DeRow<'a>) -> Self {
        let mut prefix = de_row.prefix.join("_");
        if !prefix.is_empty() {
            prefix.push('_');
        }

        let keys = de_row
            .cols
            .iter()
            .map(|c| c.name())
            .filter(|c| c.starts_with(&prefix))
            .map(|c| c[prefix.len()..].to_owned())
            .collect::<Vec<_>>()
            .into_iter();
        let values = de_row
            .cols
            .iter()
            .map(|c| c.name())
            .filter(|c| c.starts_with(&prefix))
            .collect::<Vec<_>>()
            .into_iter();
        Self {
            de_row,
            keys,
            values,
        }
    }
}
