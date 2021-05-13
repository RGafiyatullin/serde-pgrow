use super::*;

pub struct DeJsValue<'a> {
    pub(super) value: &'a JsValue,
}

impl<'a> DeJsValue<'a> {
    pub(super) fn new(value: &'a JsValue) -> Self {
        Self { value }
    }
}
