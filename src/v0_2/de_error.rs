use std::error::Error as StdError;
use std::fmt;

use ::serde::de::Error as DeError;
use ::tokio_postgres::Error as PgError;

#[derive(Debug)]
pub enum Error {
    Custom(String),
    PgError(PgError),
    UnexpectedNull(String),
    UnsupportedType,
    Unimplemented,
}

impl From<PgError> for Error {
    fn from(pg_error: PgError) -> Self {
        Self::PgError(pg_error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl StdError for Error {}

impl DeError for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Self::Custom(format!("{}", msg))
    }
}
