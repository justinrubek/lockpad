use std::str::FromStr;

use crate::error::Result;
use axum::Json;
use lockpad_models::{
    application::{Application, Builder as ApplicationBuilder},
    entity::Builder,
};
use scylla_dynamodb::entity::{PrimaryId, PutEntity, QueryEntity};

/// Performs a dynamodb query to list all users.
pub(crate) async fn list_applications(
    dynamodb: axum::extract::State<scylla_dynamodb::DynamodbTable>,
    // owner_id: axum::extract::Path<PrimaryId>,
) -> Result<Json<Vec<Application>>> {
    let owner_id = PrimaryId::from_str("01GV6VJZ9N5FYYX9XC3VA50N3C")?;

    let res = Application::query(&dynamodb, owner_id)?.send().await?;

    tracing::debug!(?res, "query result");

    let items = res.items().map(|slice| slice.to_vec()).unwrap();
    let items = items
        .into_iter()
        .map(|item| {
            let item: Application = serde_dynamo::from_item(item).unwrap();
            item
        })
        .collect::<Vec<_>>();

    Ok(Json(items))
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateApplication {
    pub name: String,
}

pub(crate) async fn create_application(
    dynamodb: axum::extract::State<scylla_dynamodb::DynamodbTable>,
    payload: axum::extract::Json<CreateApplication>,
) -> Result<Json<Application>> {
    // For now, use a dummy owner_id.
    let owner_id = PrimaryId::new();

    let item = ApplicationBuilder::default()
        .name(payload.0.name.to_owned())
        .owner_id(owner_id)
        .build()?;
    tracing::info!(?item, "creating application");

    let item = item.put_item(&dynamodb)?.send().await?;
    tracing::info!(?item, "created application");

    let attributes = item.attributes().unwrap();
    let item = serde_dynamo::from_item(attributes.to_owned()).unwrap();

    Ok(Json(item))
}
