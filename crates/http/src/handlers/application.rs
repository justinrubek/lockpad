use std::str::FromStr;

use crate::{
    error::{Error, Result},
    ServerState,
};
use axum::{extract::State, Json};
use lockpad_models::{
    application::{Application, Builder as ApplicationBuilder},
    entity::Builder,
};

pub(crate) async fn list_applications(
    State(ServerState { pg_pool, .. }): State<ServerState>,
    claims: lockpad_auth::Claims,
) -> Result<Json<Vec<Application>>> {
    let owner_id = lockpad_ulid::Ulid::from_str(&claims.sub)?;
    let pagination = lockpad_models::Pagination {
        last_key: None,
        count: 10,
    };

    let query = Application::query(&pg_pool, owner_id, pagination).await?;
    let (items, _pagination) = query;

    Ok(Json(items))
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateApplication {
    pub name: String,
    pub allowed_origins: Vec<String>,
    pub allowed_callback_urls: Vec<String>,
}

pub(crate) async fn create_application(
    State(ServerState { pg_pool, .. }): State<ServerState>,
    claims: lockpad_auth::Claims,
    payload: axum::extract::Json<CreateApplication>,
) -> Result<Json<Application>> {
    let owner_id = lockpad_ulid::Ulid::from_str(&claims.sub)?;

    let item = ApplicationBuilder::default()
        .name(payload.0.name.to_owned())
        .owner_id(owner_id)
        .allowed_origins(payload.0.allowed_origins)
        .allowed_callback_urls(payload.0.allowed_callback_urls)
        .build()?;

    item.create(&pg_pool).await?;

    tracing::info!(?item, "created application");
    Ok(Json(item))
}

pub(crate) async fn get_application(
    State(ServerState { pg_pool, .. }): State<ServerState>,
    claims: lockpad_auth::Claims,
    application_id: axum::extract::Path<lockpad_ulid::Ulid>,
) -> Result<Json<Application>> {
    let owner_id = lockpad_ulid::Ulid::from_str(&claims.sub)?;

    let item = Application::by_id(&pg_pool, &application_id).await?;
    let item = item.ok_or(Error::NotFound)?;

    if item.owner_id != owner_id {
        return Err(Error::NotFound);
    }

    Ok(Json(item))
}
