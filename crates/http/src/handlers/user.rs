use crate::{error::Result, ServerState};
use axum::{extract::State, Json};
use lockpad_models::user;
use lockpad_ulid::Ulid;

pub(crate) async fn list_users(
    State(ServerState { pg_pool, .. }): State<ServerState>,
) -> Result<Json<Vec<user::User>>> {
    let pagination = lockpad_models::Pagination {
        last_key: None,
        count: 10,
    };

    let (users, _pagination) = user::User::query(&pg_pool, pagination).await?;

    tracing::debug!(?users, "query result");

    Ok(Json(users))
}

pub(crate) async fn get_user(
    State(ServerState { pg_pool, .. }): State<ServerState>,
    user_id: axum::extract::Path<Ulid>,
) -> Result<Json<user::User>> {
    tracing::debug!(?user_id, "getting user");

    let user = user::User::by_id(&pg_pool, &user_id.0).await?;
    match user {
        Some(user) => Ok(Json(user)),
        None => Err(crate::error::Error::NotFound),
    }
}
