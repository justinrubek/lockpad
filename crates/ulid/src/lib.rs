use sqlx::{encode::IsNull, postgres::PgHasArrayType, Decode, Encode};
pub mod error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ulid(rusty_ulid::Ulid);

impl Ulid {
    pub fn generate() -> Self {
        Ulid(rusty_ulid::Ulid::generate())
    }
}

impl From<Ulid> for rusty_ulid::Ulid {
    fn from(ulid: Ulid) -> Self {
        ulid.0
    }
}

impl std::str::FromStr for Ulid {
    type Err = rusty_ulid::DecodingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        rusty_ulid::Ulid::from_str(s).map(Ulid)
    }
}

impl std::fmt::Display for Ulid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl sqlx::Type<sqlx::Postgres> for Ulid {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("ulid")
    }
}

impl PgHasArrayType for Ulid {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_ulid")
    }
}

impl Encode<'_, sqlx::Postgres> for Ulid {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> IsNull {
        let bytes: [u8; 16] = self.0.into();
        buf.extend_from_slice(&bytes);
        IsNull::No
    }
}

impl Decode<'_, sqlx::Postgres> for Ulid {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        let bytes = value.as_bytes()?;
        let ulid = rusty_ulid::Ulid::try_from(bytes)?;
        Ok(Ulid(ulid))
    }
}

impl std::convert::From<Ulid> for sqlx::types::Uuid {
    fn from(ulid: Ulid) -> Self {
        let bytes: [u8; 16] = ulid.0.into();
        sqlx::types::Uuid::from_bytes(bytes)
    }
}
