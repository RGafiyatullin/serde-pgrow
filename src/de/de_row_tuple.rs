use super::*;

#[derive(Debug)]
pub struct DeRowTuple<'a> {
    pub(super) de_row: &'a DeRow<'a>,
    #[allow(unused)]
    pub(super) struct_name: Option<&'static str>,
    pub(super) keys: std::ops::Range<usize>,
}

impl<'a> DeRowTuple<'a> {
    pub(super) fn new(
        de_row: &'a DeRow<'a>,
        struct_name: Option<&'static str>,
        len: usize,
    ) -> Self {
        Self {
            de_row,
            struct_name,
            keys: 0..len,
        }
    }
}
