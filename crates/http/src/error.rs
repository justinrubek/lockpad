#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Hyper(#[from] hyper::Error),
    #[error(transparent)]
    Argon2(#[from] argon2::password_hash::Error),

    #[error(transparent)]
    LockpadModels(#[from] lockpad_models::error::Error),
    #[error(transparent)]
    LockpadDynamodb(#[from] scylla_dynamodb::error::Error),

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
    DynamodbScan(#[from] aws_sdk_dynamodb::types::SdkError<aws_sdk_dynamodb::error::ScanError>),

    #[error("unauthorized")]
    Unauthorized,
}

pub type Result<T> = std::result::Result<T, Error>;

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::info!(?self, "error response");
        let status = match self {
            Error::Unauthorized => axum::http::StatusCode::UNAUTHORIZED,
            _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}
