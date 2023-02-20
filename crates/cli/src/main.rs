use serde::{Deserialize, Serialize};
use serde_dynamo::to_item;

#[derive(Debug, Serialize, Deserialize)]
pub struct PrimaryId {
    pub pk: String,
    pub sk: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(flatten)]
    pub id: PrimaryId,

    pub name: String,
}

// An opinionated table creation function.
// Creates a table with composite keys pk and sk
async fn create_table(
    client: &aws_sdk_dynamodb::Client,
    table_name: &str,
) -> Result<(), aws_sdk_dynamodb::Error> {
    let primary_key = aws_sdk_dynamodb::model::AttributeDefinition::builder()
        .attribute_name("pk".to_string())
        .attribute_type(aws_sdk_dynamodb::model::ScalarAttributeType::S)
        .build();
    let sort_key = aws_sdk_dynamodb::model::AttributeDefinition::builder()
        .attribute_name("sk".to_string())
        .attribute_type(aws_sdk_dynamodb::model::ScalarAttributeType::S)
        .build();

    let primary_key_schema = aws_sdk_dynamodb::model::KeySchemaElement::builder()
        .attribute_name("pk".to_string())
        .key_type(aws_sdk_dynamodb::model::KeyType::Hash)
        .build();

    let sort_key_schema = aws_sdk_dynamodb::model::KeySchemaElement::builder()
        .attribute_name("sk".to_string())
        .key_type(aws_sdk_dynamodb::model::KeyType::Range)
        .build();

    client
        .create_table()
        .table_name(table_name)
        .set_attribute_definitions(Some(vec![primary_key, sort_key]))
        .set_key_schema(Some(vec![primary_key_schema, sort_key_schema]))
        .billing_mode(aws_sdk_dynamodb::model::BillingMode::PayPerRequest)
        .send()
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = scylla_dynamodb::connect_dynamodb("http://localhost:8100".to_string()).await;
    create_table(&client, "users").await?;

    let all_tables = client.list_tables().send().await?;
    println!("{:?}", all_tables);

    let user = User {
        id: PrimaryId {
            pk: "pk".to_string(),
            sk: "sk".to_string(),
        },
        name: "name".to_string(),
    };
    println!("{:?}", user);
    let item = to_item(user)?;
    client
        .put_item()
        .table_name("users")
        .set_item(Some(item))
        .send()
        .await?;

    let all_items = client.scan().table_name("users").send().await?;

    println!("{:?}", all_items);

    Ok(())
}
