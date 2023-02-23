use crate::{
    entity::{EntityPrefix, PrimaryId, UniqueEntity},
    error::Result,
};
use serde::{Deserialize, Serialize};

/* TODO:
 * - Implement types for interacting with the user model
 * This should include:
 *
 * - User with all fields, including identity and credentials
 * - Create user, which is a subset of the user model
 * - Update user, which is a subset of the user model
 *   - Updates may need to consider partial and full updates
 * - Provide some way of serializing this to dynamodb format
 */

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserData {
    pub identifier: String,
    pub secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: PrimaryId,

    #[serde(flatten)]
    pub data: UserData,
}

impl User {
    pub fn new(data: UserData) -> Self {
        Self {
            id: PrimaryId::new(),
            data,
        }
    }
}

impl UniqueEntity for User {
    fn unique_field(
        &self,
        fields: std::collections::HashMap<String, aws_sdk_dynamodb::model::AttributeValue>,
    ) -> Result<Option<aws_sdk_dynamodb::model::AttributeValue>> {
        let field = fields.get("identifier").unwrap();
        Ok(Some(field.clone()))
    }
}

impl EntityPrefix for User {
    const PREFIX: &'static str = "user";
}

/// Database representation of a user.
/// This is the format that is used to interact with the database.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DbRepr {
    pub pk: String,
    pub sk: String,
    pub id: PrimaryId,
    pub identifier: String,
    pub secret: String,
}
