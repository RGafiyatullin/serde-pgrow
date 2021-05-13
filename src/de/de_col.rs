use super::*;

#[derive(Debug)]
pub struct DeCol<'a> {
    pub(super) row: &'a PgRow,
    pub(super) col: &'a PgCol,
}

impl<'a> DeCol<'a> {
    pub(crate) fn new(row: &'a PgRow, col: &'a PgCol) -> Self {
        Self { row, col }
    }
}
