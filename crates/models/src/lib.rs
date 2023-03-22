use lockpad_ulid::Ulid;
use serde::{Deserialize, Serialize};

pub mod application;
pub mod entity;
pub mod error;
pub mod user;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub last_key: Option<Ulid>,
    pub count: usize,
}

#[derive(Debug)]
pub struct User {
    pub user_id: Option<lockpad_ulid::Ulid>,
    // pub user_id: Option<sqlx::types::Uuid>,
    pub identifier: String,
}
