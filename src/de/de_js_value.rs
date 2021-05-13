use super::*;

pub struct DeJsValue {
    pub(super) value: JsValue,
}

impl DeJsValue {
    pub(super) fn new(value: JsValue) -> Self {
        Self { value }
    }
}
