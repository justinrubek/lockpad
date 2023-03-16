use crate::error::Result;
use axum::{
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    response::{IntoResponse, Response},
};
use base64::Engine;
use jsonwebtoken::{
    encode,
    jwk::{AlgorithmParameters, RSAKeyParameters},
    Algorithm, DecodingKey, EncodingKey, Header,
};
use rsa::{pkcs1::DecodeRsaPublicKey, PublicKeyParts};
use serde::{Deserialize, Serialize};

pub mod error;

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

/// The binary representation of an Ed25519 public key
/// This is used to verify JWT claims
#[derive(Clone)]
pub struct PublicKey {
    raw: Vec<u8>,
    key: DecodingKey,
    rsa_key: rsa::RsaPublicKey,
}

impl PublicKey {
    /// Create a new public key from a binary DER representation
    pub fn new(raw: Vec<u8>) -> Result<Self> {
        let contents = raw.clone();
        let str = std::str::from_utf8(&contents)?;
        Ok(Self {
            key: DecodingKey::from_rsa_pem(&raw)?,
            raw,
            rsa_key: rsa::RsaPublicKey::from_pkcs1_pem(str)?,
        })
    }
}

impl From<PublicKey> for jsonwebtoken::jwk::Jwk {
    fn from(key: PublicKey) -> Self {
        let common = jsonwebtoken::jwk::CommonParameters {
            public_key_use: Some(jsonwebtoken::jwk::PublicKeyUse::Signature),
            key_id: Some("1".to_string()),
            algorithm: Some(Algorithm::RS256),
            ..Default::default()
        };

        let n = key.rsa_key.n().to_bytes_be();
        let e = key.rsa_key.e().to_bytes_be();

        let engine = base64::engine::general_purpose::URL_SAFE_NO_PAD;
        let n_base64 = engine.encode(n);
        let e_base64 = engine.encode(e);

        let algorithm = AlgorithmParameters::RSA(RSAKeyParameters {
            key_type: jsonwebtoken::jwk::RSAKeyType::RSA,
            n: n_base64,
            e: e_base64,
        });

        jsonwebtoken::jwk::Jwk { common, algorithm }
    }
}

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        &self.raw
    }
}

impl AsRef<DecodingKey> for PublicKey {
    fn as_ref(&self) -> &DecodingKey {
        &self.key
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
