pub mod credentials;
pub mod entity;
pub mod error;

/// Connects to ScyllaDB's DynamoDB API.
/// This uses [DummyCredentialsProvider](crate::credentials::DummyCredentialsProvider) to provide dummy credentials.
pub async fn connect_dynamodb(url: String) -> aws_sdk_dynamodb::Client {
    let config = aws_sdk_dynamodb::config::Builder::new()
        .region(aws_sdk_dynamodb::Region::from_static("none"))
        .endpoint_url(url)
        .credentials_provider(crate::credentials::DummyCredentialsProvider)
        .build();

    aws_sdk_dynamodb::Client::from_conf(config)
}

#[derive(Clone, Debug)]
pub struct DynamodbTable {
    pub name: String,
    pub client: aws_sdk_dynamodb::Client,
}
