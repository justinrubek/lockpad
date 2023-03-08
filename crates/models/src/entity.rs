use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrimaryId(ulid::Ulid);

impl Default for PrimaryId {
    fn default() -> Self {
        Self(ulid::Ulid::new())
    }
}

impl PrimaryId {
    pub fn new() -> Self {
        Self::default()
    }
}

pub trait Builder {
    type Item;

    fn build(self) -> Result<Self::Item>;
}
