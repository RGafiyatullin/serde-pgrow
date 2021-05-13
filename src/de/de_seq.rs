use super::*;

#[derive(Debug)]
pub struct DeSeq<'a> {
    pub(super) elements: Vec<Option<AnyPgType<'a>>>,
}

impl<'a> DeSeq<'a> {
    fn new<I>(items: I) -> Self
    where
        I: IntoIterator<Item = Option<AnyPgType<'a>>>,
    {
        Self {
            elements: items.into_iter().collect(),
        }
    }
}
