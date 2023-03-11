use crate::error::{Error, Result};
use lockpad_derive::UniqueEntity;
use scylla_dynamodb::entity::{PrefixedEntity, PrimaryId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, UniqueEntity)]
pub struct User {
    pub id: PrimaryId,

    #[unique_id]
    pub identifier: String,
    pub secret: String,
}

impl User {
    pub fn builder() -> Builder {
        Builder::default()
    }
}

impl PrefixedEntity for User {
    const PREFIX: &'static str = "user";
}

#[derive(Debug, Default)]
pub struct Builder {
    identifier: Option<String>,
    secret: Option<String>,
}

impl Builder {
    pub fn identifier(mut self, identifier: String) -> Self {
        self.identifier = Some(identifier);
        self
    }

    pub fn secret(mut self, secret: String) -> Self {
        self.secret = Some(secret);
        self
    }
}

impl crate::entity::Builder for Builder {
    type Item = User;

    fn build(self) -> Result<Self::Item> {
        let identifier = self
            .identifier
            .ok_or_else(|| Error::ModelFieldsMissing("identifier"))?;
        let secret = self
            .secret
            .ok_or_else(|| Error::ModelFieldsMissing("secret"))?;

        Ok(User {
            id: PrimaryId::new(),
            identifier,
            secret,
        })
    }
}
