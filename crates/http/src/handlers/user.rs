use crate::error::Result;
use aws_sdk_dynamodb::model::AttributeValue;
use axum::Json;
use lockpad_models::{entity::EntityPrefix, user};

/// Performs a dynamodb query to list all users.
pub(crate) async fn list_users(
    dynamodb: axum::extract::State<scylla_dynamodb::DynamodbTable>,
) -> Result<Json<Vec<user::User>>> {
    let client = &dynamodb.client;

    let res = client
        .query()
        .table_name(&dynamodb.name)
        .key_condition_expression("#pk = :pk")
        .expression_attribute_names("#pk", "pk")
        .expression_attribute_values(":pk", AttributeValue::S(user::User::PREFIX.to_string()))
        .send()
        .await?;

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
