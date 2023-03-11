use crate::error::Result;
use aws_sdk_dynamodb::model::AttributeValue;
use axum::extract::Json;

/// Performs a dynamodb query to list all users.
pub(crate) async fn wipe_table(
    dynamodb: axum::extract::State<scylla_dynamodb::DynamodbTable>,
) -> Result<()> {
    tracing::info!("Wiping table");
    let client = &dynamodb.client;

    let res = client.scan().table_name(&dynamodb.name).send().await?;

    tracing::debug!(?res, "scan result");

    let items = res.items().map(|slice| slice.to_vec()).unwrap();

    // Delete all items (pk and sk for keys)
    for item in items {
        let pk = item.get("pk").unwrap().as_s().unwrap();
        let sk = item.get("sk").unwrap().as_s().unwrap();

        let res = client
            .delete_item()
            .table_name(&dynamodb.name)
            .key("pk", AttributeValue::S(pk.to_string()))
            .key("sk", AttributeValue::S(sk.to_string()))
            .send()
            .await?;

        tracing::debug!(?res, "delete result");
    }

    Ok(())
}

pub(crate) async fn scan_table(
    dynamodb: axum::extract::State<scylla_dynamodb::DynamodbTable>,
) -> Result<Json<Vec<serde_json::Value>>> {
    tracing::info!("Scanning table");
    let client = &dynamodb.client;

    let res = client.scan().table_name(&dynamodb.name).send().await?;

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
