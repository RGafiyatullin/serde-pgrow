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

mod de_field_name;
pub use de_field_name::DeFieldName;

mod de_row_struct;
mod de_row_struct_ma;
pub use de_row_struct::DeRowStruct;

mod de_row_tuple;
mod de_row_tuple_sa;
pub use de_row_tuple::DeRowTuple;

mod de_row_map;
mod de_row_map_ma;
pub use de_row_map::DeRowMap;

mod de_seq;
mod de_seq_sa;
pub use de_seq::DeSeq;

mod pg_any;
mod pg_any_deserializer;
pub use pg_any::PgAny;

mod pg_any_opt;
mod pg_any_opt_deserializer;
pub use pg_any_opt::PgAnyOpt;
