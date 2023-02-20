use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrimaryId {
    pub pk: String,
    pub sk: String,
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
