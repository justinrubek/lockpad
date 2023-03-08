use crate::error::Result;
use aws_sdk_dynamodb::model::AttributeValue;
use axum::Json;
use lockpad_models::user;
use scylla_dynamodb::entity::{PrefixedEntity, QueryEntity};

/// Performs a dynamodb query to list all users.
pub(crate) async fn list_users(
    dynamodb: axum::extract::State<scylla_dynamodb::DynamodbTable>,
) -> Result<Json<Vec<user::User>>> {
    let client = &dynamodb.client;

    let res = user::User::query(&dynamodb)?.send().await?;

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
