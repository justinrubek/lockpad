#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    CrockfordDecode(#[from] rusty_ulid::crockford::DecodingError),
}

pub type Result<T> = std::result::Result<T, Error>;
