#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Hyper(#[from] hyper::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("failed to hash")]
    Argon2(argon2::password_hash::Error),

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
}

pub type Result<T> = std::result::Result<T, Error>;

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        #[allow(clippy::match_single_binding)]
        let status = match self {
            _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}
