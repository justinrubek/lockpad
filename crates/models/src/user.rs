use crate::entity::PrimaryId;
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

    pub salt: String,

    #[serde(flatten)]
    pub data: UserData,
}

impl User {
    pub const PREFIX: &'static str = "user";

    pub fn new(salt: String, data: UserData) -> Self {
        Self {
            id: PrimaryId::new(),
            salt,
            data,
        }
    }
}
