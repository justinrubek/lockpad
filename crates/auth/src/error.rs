#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    TimeError(#[from] std::time::SystemTimeError),
    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error(transparent)]
    RsaPkcs1Error(#[from] rsa::pkcs1::Error),
    #[error(transparent)]
    RsaError(#[from] rsa::errors::Error),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error(transparent)]
    DecodeError(#[from] base64::DecodeError),
}

pub type Result<T> = std::result::Result<T, Error>;

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::info!(?self, "error response");
        let status = match self {
            Error::JwtError(_) => axum::http::StatusCode::UNAUTHORIZED,
            _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}
