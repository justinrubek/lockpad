use crate::entity::PrimaryId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(flatten)]
    pub id: PrimaryId,

    pub name: String,
}
