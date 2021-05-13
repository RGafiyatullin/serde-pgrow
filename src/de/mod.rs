use ::serde::de::DeserializeSeed;
use ::serde::de::MapAccess;
use ::serde::de::SeqAccess;
use ::serde::de::Visitor;
use ::serde::Deserializer;

use crate::pg::*;

mod util;

mod de_error;
pub use de_error::PgDeError;

mod de_row;
mod de_row_deserializer;
pub use de_row::DeRow;

mod de_col;
mod de_col_deserializer;
pub use de_col::DeCol;

mod de_field_name;
pub use de_field_name::DeFieldName;

mod de_row_struct;
mod de_row_struct_ma;
pub use de_row_struct::DeRowStruct;

mod de_row_tuple;
mod de_row_tuple_sa;
pub use de_row_tuple::DeRowTuple;
