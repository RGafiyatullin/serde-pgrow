pub use ::tokio_postgres::types::Type as PgType;
pub use ::tokio_postgres::Column as PgCol;
pub use ::tokio_postgres::Row as PgRow;

#[derive(Debug)]
pub struct AnyPgType<'a> {
    pg_type: PgType,
    raw_data: &'a [u8],
}
impl<'a> tokio_postgres::types::FromSql<'a> for AnyPgType<'a> {
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

pub struct UnitPgType;
impl<'a> tokio_postgres::types::FromSql<'a> for UnitPgType {
    fn from_sql(
        _ty: &PgType,
        _raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        Ok(Self)
    }

    fn accepts(_ty: &tokio_postgres::types::Type) -> bool {
        true
    }
}
