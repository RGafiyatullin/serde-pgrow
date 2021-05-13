use super::*;

#[derive(Debug)]
pub struct PgAny<'a> {
    pub(super) pg_type: PgType,
    pub(super) raw_data: &'a [u8],
}
impl<'a> tokio_postgres::types::FromSql<'a> for PgAny<'a> {
    fn from_sql(
        pg_type: &PgType,
        raw_data: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        Ok(Self {
            pg_type: pg_type.to_owned(),
            raw_data,
        })
    }
    fn accepts(_ty: &PgType) -> bool {
        true
    }
}
