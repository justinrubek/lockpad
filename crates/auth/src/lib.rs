use crate::error::Result;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    response::{IntoResponse, Response},
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};

pub mod error;
pub mod key;

pub use key::PublicKey;

/// The claims of a JWT
#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

impl Claims {
    /// Create a claim with a given subject
    /// The expiration time is set to 7 days from the moment of creation
    pub fn new(sub: String) -> Self {
        let now = std::time::SystemTime::now();
        let iat = now.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as usize;

        let token_life = std::time::Duration::from_secs(60 * 60 * 24 * 7); // 7 days
        let exp = (now + token_life)
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        Self { sub, exp, iat }
    }

    /// Encode the claims into a JWT string
    pub async fn encode(&self, key: &EncodingKey) -> Result<String> {
        let header = Header::new(Algorithm::RS256);
        let token = encode(&header, self, key)?;

        Ok(token)
    }

    pub async fn decode(token: &str, key: &DecodingKey) -> Result<Self> {
        let validation = jsonwebtoken::Validation::new(Algorithm::RS256);
        let claims = jsonwebtoken::decode::<Self>(token, key, &validation)?;

        Ok(claims.claims)
    }

    pub async fn decode_validation(
        token: &str,
        key: &DecodingKey,
        validation: &jsonwebtoken::Validation,
    ) -> Result<Self> {
        let claims = jsonwebtoken::decode::<Self>(token, key, validation)?;

        Ok(claims.claims)
    }
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
    key::PublicKey: axum::extract::FromRef<S>,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        // Extract the authorization header
        let TypedHeader(Authorization(token)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|err| err.into_response())?;

        // Verify the token
        let key: PublicKey = FromRef::from_ref(state);
        let key: DecodingKey = FromRef::from_ref(&key);
        let claims = Claims::decode(token.token(), &key)
            .await
            .map_err(|err| err.into_response())?;

        Ok(claims)
    }
}
