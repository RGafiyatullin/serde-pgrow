pub use ::tokio_postgres::types::Type as PgType;
pub use ::tokio_postgres::Column as PgCol;
pub use ::tokio_postgres::Row as PgRow;

pub struct UnitPgType;
impl<'a> tokio_postgres::types::FromSql<'a> for UnitPgType {
    fn from_sql(
        _ty: &tokio_postgres::types::Type,
        _raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        Ok(Self)
    }

    fn accepts(_ty: &tokio_postgres::types::Type) -> bool {
        true
    }
}
