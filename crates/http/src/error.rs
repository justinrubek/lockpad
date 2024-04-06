#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Hyper(#[from] hyper::Error),
    #[error(transparent)]
    Argon2(#[from] argon2::password_hash::Error),
    #[error(transparent)]
    Jsonwebtoken(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Time(#[from] std::time::SystemTimeError),

    #[error(transparent)]
    LockpadModels(#[from] lockpad_models::error::Error),
    #[error(transparent)]
    LockpadAuth(#[from] lockpad_auth::error::Error),
    #[error(transparent)]
    LockpadUlid(#[from] lockpad_ulid::error::Error),

    #[error("Failed to build server struct")]
    ServerBuilder,

    #[error("unauthorized")]
    Unauthorized,
    #[error("not found")]
    NotFound,

    #[error(transparent)]
    AxumFormRejection(#[from] axum::extract::rejection::FormRejection),
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
}

pub type Result<T> = std::result::Result<T, Error>;

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::warn!(?self, "error response");
        let status = match self {
            Error::Unauthorized => axum::http::StatusCode::UNAUTHORIZED,
            Error::NotFound => axum::http::StatusCode::NOT_FOUND,

            Error::AxumFormRejection(_) => axum::http::StatusCode::BAD_REQUEST,
            Error::ValidationError(_) => {
                let message = format!("input validation error: [{self}]").replace('\n', ", ");
                return (axum::http::StatusCode::BAD_REQUEST, message).into_response();
            }

            _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}
