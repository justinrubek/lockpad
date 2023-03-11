use crate::error::{Error, Result};
use lockpad_derive::OwnedEntity;
use scylla_dynamodb::entity::{PrefixedEntity, PrimaryId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, OwnedEntity)]
pub struct Application {
    #[owner_id]
    pub owner_id: PrimaryId,
    #[object_id]
    pub object_id: PrimaryId,

    pub name: String,
}

impl Application {
    pub fn builder() -> Builder {
        Builder::default()
    }
}

impl PrefixedEntity for Application {
    const PREFIX: &'static str = "app";
}

#[derive(Debug, Default)]
pub struct Builder {
    name: Option<String>,
    owner_id: Option<PrimaryId>,
}

impl Builder {
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn owner_id(mut self, owner_id: PrimaryId) -> Self {
        self.owner_id = Some(owner_id);
        self
    }
}

impl crate::entity::Builder for Builder {
    type Item = Application;

    fn build(self) -> Result<Self::Item> {
        let name = self.name.ok_or_else(|| Error::ModelFieldsMissing("name"))?;
        let owner_id = self
            .owner_id
            .ok_or_else(|| Error::ModelFieldsMissing("owner_id"))?;

        Ok(Application {
            name,
            owner_id,
            object_id: PrimaryId::new(),
        })
    }
}
