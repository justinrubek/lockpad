use crate::error::Result;
use axum::{
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};

pub mod error;
pub mod key;

pub use key::PublicKey;

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
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync + AsRef<PublicKey>,
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

        println!("Got token: {}", token.token());

        // Verify the token
        let key = state.as_ref(); // Get the public key from the state
        let key = key.as_ref(); // Get the decoding key from the public key
        let claims = Claims::decode(token.token(), key)
            .await
            .map_err(|err| err.into_response())?;

        Ok(claims)
    }
}
