use crate::error::Result;

pub trait Builder {
    type Item;

    fn build(self) -> Result<Self::Item>;
}
