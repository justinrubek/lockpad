use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlx::{
    encode::IsNull,
    postgres::{PgHasArrayType, PgValueFormat},
    Decode, Encode,
};
use std::str::FromStr;
pub mod error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Ulid(rusty_ulid::Ulid);

impl Ulid {
    pub fn generate() -> Self {
        Ulid(rusty_ulid::Ulid::generate())
    }

    /// Exposes the value in the format sqlx can use in a text query
    pub fn queryable(&self) -> String {
        sqlx::types::Uuid::from(*self).to_string()
    }

    pub fn to_inner(&self) -> rusty_ulid::Ulid {
        self.0
    }

    pub fn to_sqlx_uuid(&self) -> sqlx::types::Uuid {
        sqlx::types::Uuid::from(*self)
    }
}

impl From<Ulid> for rusty_ulid::Ulid {
    fn from(ulid: Ulid) -> Self {
        ulid.0
    }
}

impl std::str::FromStr for Ulid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        rusty_ulid::Ulid::from_str(s).map(Ulid).map_err(Error::from)
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

    fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
        // *ty == Self::type_info()
        *ty == Self::type_info()
            || <sqlx::types::Uuid as sqlx::Type<sqlx::Postgres>>::compatible(ty)
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
    fn decode(
        value: sqlx::postgres::PgValueRef<'_>,
    ) -> std::result::Result<Self, sqlx::error::BoxDynError> {
        Ok(match value.format() {
            PgValueFormat::Binary => {
                let bytes = value.as_bytes()?;
                let ulid = rusty_ulid::Ulid::try_from(bytes)?;
                Ulid(ulid)
            }
            PgValueFormat::Text => {
                let s = value.as_str()?;
                let ulid = rusty_ulid::Ulid::from_str(s)?;
                Ulid(ulid)
            }
        })
    }
}

impl std::convert::From<Ulid> for sqlx::types::Uuid {
    fn from(ulid: Ulid) -> Self {
        let bytes: [u8; 16] = ulid.0.into();
        sqlx::types::Uuid::from_bytes(bytes)
    }
}

impl std::convert::From<sqlx::types::Uuid> for Ulid {
    fn from(uuid: sqlx::types::Uuid) -> Self {
        let bytes = uuid.as_bytes().as_slice();
        // gather bytes as u128
        let mut arr_bytes: [u8; 16] = [0; 16];
        arr_bytes.clone_from_slice(bytes);

        let uuid: u128 = u128::from_be_bytes(arr_bytes);

        let ulid = rusty_ulid::Ulid::from(uuid);
        Ulid(ulid)
    }
}
