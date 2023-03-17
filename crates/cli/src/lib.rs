pub mod config;
// An opinionated table creation function.
// Creates a table with composite keys pk and sk
pub async fn create_table(
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
