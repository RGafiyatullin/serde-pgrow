use crate::de::PgDeError;
use crate::pg::*;

pub fn ensure_pg_type(col: &PgCol, types: &[PgType]) -> Result<(), PgDeError> {
    if types.into_iter().any(|t| t == col.type_()) {
        Ok(())
    } else {
        Err(PgDeError::UnsupportedType(col.type_().to_owned()))
    }
}
