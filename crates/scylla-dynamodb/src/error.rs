#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    DynamodbQuery(#[from] aws_sdk_dynamodb::types::SdkError<aws_sdk_dynamodb::error::QueryError>),
    #[error(transparent)]
    DynamodbPut(#[from] aws_sdk_dynamodb::types::SdkError<aws_sdk_dynamodb::error::PutItemError>),
    #[error(transparent)]
    DynamodbGet(#[from] aws_sdk_dynamodb::types::SdkError<aws_sdk_dynamodb::error::GetItemError>),
    #[error(transparent)]
    DynamodbDelete(
        #[from] aws_sdk_dynamodb::types::SdkError<aws_sdk_dynamodb::error::DeleteItemError>,
    ),
    #[error(transparent)]
    DynamodbUpdate(
        #[from] aws_sdk_dynamodb::types::SdkError<aws_sdk_dynamodb::error::UpdateItemError>,
    ),

    #[error(transparent)]
    SerdeDynamo(#[from] serde_dynamo::Error),

    #[error(transparent)]
    UlidDecode(#[from] ulid::DecodeError),
}

pub type Result<T> = std::result::Result<T, Error>;
