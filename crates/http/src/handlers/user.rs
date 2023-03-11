use crate::error::Result;
use axum::Json;
use lockpad_models::user;
use scylla_dynamodb::entity::{FormatKey, GetEntity, QueryEntity};

/// Performs a dynamodb query to list all users.
pub(crate) async fn list_users(
    dynamodb: axum::extract::State<scylla_dynamodb::DynamodbTable>,
) -> Result<Json<Vec<user::User>>> {
    let res = user::User::query(&dynamodb, ())?.send().await?;

    tracing::debug!(?res, "query result");

    let items = res.items().map(|slice| slice.to_vec()).unwrap();

    let users = items
        .into_iter()
        .map(|item| {
            let user: user::User = serde_dynamo::from_item(item).unwrap();
            user
        })
        .collect::<Vec<_>>();

    Ok(Json(users))
}

pub(crate) async fn get_user(
    dynamodb: axum::extract::State<scylla_dynamodb::DynamodbTable>,
    user_id: axum::extract::Path<String>,
) -> Result<Json<user::User>> {
    let user_id = user_id.to_string();
    tracing::info!(?user_id, "getting user");

    let key = user::User::format_key(user_id);

    let res = user::User::get(&dynamodb, key)?.send().await?;

    tracing::info!(?res, "query result");

    let item = res.item().unwrap();

    Ok(Json(serde_dynamo::from_item(item.to_owned()).unwrap()))
}
