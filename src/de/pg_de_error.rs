use std::error::Error as StdError;

use ::serde::de::Error as DeError;
use ::tokio_postgres::Error as PgError;

use crate::pg::PgType;
use crate::AnyError;

#[derive(Debug, ::thiserror::Error)]
pub enum PgDeError {
    #[error("PgDeError::Custom: {}", _0)]
    Custom(String, #[source] Option<AnyError>),

    #[error("PgDeError::PgError")]
    PgError(#[source] PgError),

    // UnexpectedNull(String),
    #[error("PgDeError::UnsupportedType: {:?}", _0)]
    UnsupportedType(PgType),

    #[error("PgDeError::CastError")]
    CastError(#[source] Box<dyn StdError + Send + Sync>),

    #[error("PgDeError::WasNull")]
    WasNull,

    #[error("PgDeError::Unimplemented: {} [{}]", _0, _1)]
    Unimplemented(&'static str, &'static str),
}

impl PgDeError {
    pub fn cast_error(e: Box<dyn StdError + Send + Sync>) -> Self {
        Self::CastError(e)
    }
}

impl From<PgError> for PgDeError {
    fn from(pg_error: PgError) -> Self {
        Self::PgError(pg_error)
    }
}

impl DeError for PgDeError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Self::Custom(format!("{}", msg), None)
    }
}
