use crate::{error::Result, ServerState};
use aws_sdk_dynamodb::model::AttributeValue;
use axum::extract::{Json, State};
use scylla_dynamodb::DynamodbTable;

/// Performs a dynamodb query to list all users.
pub(crate) async fn wipe_table(
    State(ServerState {
        dynamodb: DynamodbTable { client, name },
        ..
    }): State<ServerState>,
) -> Result<()> {
    tracing::info!("Wiping table");

    let res = client.scan().table_name(&name).send().await?;

    tracing::debug!(?res, "scan result");

    let items = res.items().map(|slice| slice.to_vec()).unwrap();

    // Delete all items (pk and sk for keys)
    for item in items {
        let pk = item.get("pk").unwrap().as_s().unwrap();
        let sk = item.get("sk").unwrap().as_s().unwrap();

        let res = client
            .delete_item()
            .table_name(&name)
            .key("pk", AttributeValue::S(pk.to_string()))
            .key("sk", AttributeValue::S(sk.to_string()))
            .send()
            .await?;

        tracing::debug!(?res, "delete result");
    }

    Ok(())
}

pub(crate) async fn scan_table(
    State(ServerState { dynamodb, .. }): State<ServerState>,
) -> Result<Json<Vec<serde_json::Value>>> {
    tracing::info!("Scanning table");
    let res = dynamodb
        .client
        .scan()
        .table_name(&dynamodb.name)
        .send()
        .await?;

    // Scan all items from dynamodb, and return them as a list of JSON objects
    let items = res.items().map(|slice| slice.to_vec()).unwrap();
    let items = items
        .into_iter()
        .map(|item| {
            let item: serde_json::Value = serde_dynamo::from_item(item).unwrap();
            item
        })
        .collect::<Vec<_>>();

    Ok(Json(items))
}
