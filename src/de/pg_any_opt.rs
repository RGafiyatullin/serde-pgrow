use super::*;

#[derive(Debug)]
pub struct PgAnyOpt<'a> {
    pub(super) inner: Option<PgAny<'a>>,
}

impl<'a> From<Option<PgAny<'a>>> for PgAnyOpt<'a> {
    fn from(inner: Option<PgAny<'a>>) -> Self {
        Self { inner }
    }
}
