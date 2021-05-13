use super::*;

#[derive(Debug)]
pub struct DeRowStruct<'a> {
    pub(super) de_row: &'a DeRow<'a>,
    pub(super) struct_name: &'static str,
    pub(super) keys: std::slice::Iter<'static, &'static str>,
    pub(super) values: std::slice::Iter<'static, &'static str>,
}

impl<'a> DeRowStruct<'a> {
    pub(super) fn new(
        de_row: &'a DeRow<'a>,
        struct_name: &'static str,
        field_names: &'static [&'static str],
    ) -> Self {
        Self {
            de_row,
            struct_name,
            keys: field_names.into_iter(),
            values: field_names.into_iter(),
        }
    }
}
