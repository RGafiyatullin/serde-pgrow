use crate::de::PgDeError;
use crate::pg::*;

pub fn ensure_pg_type(ty: &PgType, types: &[PgType]) -> Result<(), PgDeError> {
    if types.into_iter().any(|t| t == ty) {
        Ok(())
    } else {
        Err(PgDeError::UnsupportedType(ty.to_owned()))
    }
}
