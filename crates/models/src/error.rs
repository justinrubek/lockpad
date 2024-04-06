#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error("invalid unique field")]
    InvalidUniqueField,
    #[error("required fields missing")]
    ModelFieldsMissing(&'static str),
}

pub type Result<T> = std::result::Result<T, Error>;
