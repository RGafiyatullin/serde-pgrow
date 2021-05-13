mod de_error;
pub use de_error::Error;

mod pg_row_ext;

mod de;

pub mod prelude {
    pub use crate::v0_2::pg_row_ext::*;
}
