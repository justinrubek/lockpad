use aws_credential_types::{
    provider::{self, future, ProvideCredentials},
    Credentials,
};

/// Connects to ScyllaDB's DynamoDB API.
/// This uses [DummyCredentialsProvider](DummyCredentialsProvider) to provide dummy credentials.
pub async fn connect_dynamodb(url: String) -> aws_sdk_dynamodb::Client {
    let config = aws_sdk_dynamodb::config::Builder::new()
        .region(aws_sdk_dynamodb::Region::from_static("none"))
        .endpoint_url(url)
        .credentials_provider(DummyCredentialsProvider)
        .build();

    aws_sdk_dynamodb::Client::from_conf(config)
}

#[derive(Debug, Clone)]
/// Dummy credentials provider.
/// This is used to connect to ScyllaDB's DynamoDB API when using the AWS SDK.
/// The values returned are garbage, but the AWS SDK requires them to be set.
pub struct DummyCredentialsProvider;

impl DummyCredentialsProvider {
    async fn credentials(&self) -> provider::Result {
        Ok(Credentials::new("none", "none", None, None, "none"))
    }
}

impl ProvideCredentials for DummyCredentialsProvider {
    fn provide_credentials<'a>(&'a self) -> future::ProvideCredentials<'a>
    where
        Self: 'a,
    {
        future::ProvideCredentials::new(self.credentials())
    }
}
