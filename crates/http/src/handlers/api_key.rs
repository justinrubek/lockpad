use std::str::FromStr;

use crate::{
    error::{Error, Result},
    handlers::auth::hash_string,
    ServerState,
};
use axum::{extract::State, Json};
use lockpad_models::{
    api_key::{ApiKey, Builder as ApiKeyBuilder},
    entity::Builder,
};

pub(crate) async fn list_api_keys(
    State(ServerState { pg_pool, .. }): State<ServerState>,
    claims: lockpad_auth::Claims,
) -> Result<Json<Vec<ApiKey>>> {
    let owner_id = lockpad_ulid::Ulid::from_str(&claims.sub)?;
    let pagination = lockpad_models::Pagination {
        last_key: None,
        count: 10,
    };

    let query = ApiKey::query(&pg_pool, owner_id, pagination).await?;
    let (items, _pagination) = query;

    Ok(Json(items))
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateApiKey {
    pub name: String,
}

pub(crate) async fn create_api_key(
    State(ServerState { pg_pool, .. }): State<ServerState>,
    claims: lockpad_auth::Claims,
    payload: axum::extract::Json<CreateApiKey>,
) -> Result<Json<ApiKey>> {
    let owner_id = lockpad_ulid::Ulid::from_str(&claims.sub)?;

    let secret = lockpad_ulid::Ulid::generate().to_string();
    let secret_hash = hash_string(secret.as_bytes()).await?;

    let mut item = ApiKeyBuilder::default()
        .name(payload.0.name.to_owned())
        .owner_id(owner_id)
        .secret(secret_hash)
        .build()?;

    item.create(&pg_pool).await?;

    // Overwrite the secret with the unhashed version. This is the only time we will return the secret.
    item.secret = secret;

    tracing::info!(?item, "created api_key");
    Ok(Json(item))
}

pub(crate) async fn get_api_key(
    State(ServerState { pg_pool, .. }): State<ServerState>,
    claims: lockpad_auth::Claims,
    api_key_id: axum::extract::Path<lockpad_ulid::Ulid>,
) -> Result<Json<ApiKey>> {
    let owner_id = lockpad_ulid::Ulid::from_str(&claims.sub)?;

    let item = ApiKey::by_id(&pg_pool, &api_key_id).await?;
    let item = item.ok_or(Error::NotFound)?;

    if item.owner_id != owner_id {
        return Err(Error::NotFound);
    }

    Ok(Json(item))
}
