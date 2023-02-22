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

// TODO: Secret should be a type that can be serialized to a string
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SecretData {
    pub salt: String,
    pub secret: String,
}

// TODO: Determine response type for trait
// This could return a DynamoDb item hashmap
pub trait IdentityRef {
    fn id(&self) -> &PrimaryId;
}

impl IdentityRef for PrimaryId {
    fn id(&self) -> &PrimaryId {
        self
    }
}
