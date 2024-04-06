use crate::error::{Error, Result};
use axum::extract::FromRef;
use base64::Engine;
use jsonwebtoken::{
    jwk::{AlgorithmParameters, RSAKeyParameters},
    Algorithm, DecodingKey,
};
use rsa::{
    pkcs1::{DecodeRsaPublicKey, EncodeRsaPublicKey},
    PublicKeyParts, RsaPublicKey,
};

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

    pub fn parse_from_jwks(jwks_str: &str) -> Result<Vec<Self>> {
        let jwks: jsonwebtoken::jwk::JwkSet = serde_json::from_str(jwks_str)?;

        jwks.keys.into_iter().map(Self::try_from).collect()
    }

    pub fn parse_from_pem(pem_str: &str) -> Result<Self> {
        let rsa_key = rsa::RsaPublicKey::from_pkcs1_pem(pem_str)?;
        Self::try_from(rsa_key)
    }
}

impl TryFrom<rsa::RsaPublicKey> for PublicKey {
    type Error = Error;

    fn try_from(key: RsaPublicKey) -> Result<Self> {
        let data = key.to_pkcs1_pem(rsa::pkcs1::LineEnding::LF)?;
        let data = data.as_bytes().to_vec();
        Self::new(data)
    }
}

impl TryFrom<jsonwebtoken::jwk::Jwk> for PublicKey {
    type Error = Error;

    fn try_from(jwk: jsonwebtoken::jwk::Jwk) -> Result<Self> {
        match jwk.common.algorithm {
            Some(Algorithm::RS256) => {
                let rsa_params = match jwk.algorithm {
                    AlgorithmParameters::RSA(params) => params,
                    _ => unimplemented!(),
                };

                let engine = base64::engine::general_purpose::URL_SAFE_NO_PAD;

                let n_bytes = engine.decode(rsa_params.n.as_bytes())?;
                let e_bytes = engine.decode(rsa_params.e.as_bytes())?;

                let n = rsa::BigUint::from_bytes_be(&n_bytes);
                let e = rsa::BigUint::from_bytes_be(&e_bytes);

                let rsa_key = rsa::RsaPublicKey::new(n, e)?;
                Self::try_from(rsa_key)
            }
            _ => unimplemented!(),
        }
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

impl FromRef<PublicKey> for DecodingKey {
    fn from_ref(key: &PublicKey) -> Self {
        key.key.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{error::Result, Claims};

    const RSA_PUBLIC_KEY: &str = "-----BEGIN RSA PUBLIC KEY-----
MIIBCgKCAQEAwmNDtlawWXevdB5fDT4gkmBl/al/Ij9KcyAZc78O2m3Cnb9nfcSl
MuiChPP3RsehaoyXGl8dCV39gI0bEViAX5/OF5x39mFJdhe00+bUe4X7mKgItA6L
N10PeIhJR59zis8URSzVDm7OsA2HLbrRxM5k0kapNBmFN1+Us2XnNn0V2/QR9ite
Qgkpe4I5/adAwj+fQo+dY13j5N11GP7jhnEmw7MiNjo0Kv9h1ZJJHDlb9edSmA6d
g6jOxhlE4kQAJYObCF2TKhBcO+szl/zFVm9E55yiVYjWKk0R4uVXp9natMsQcbOL
OAc5t3RZmw5L0Nikhso62g9oefgwWOIPJwIDAQAB
-----END RSA PUBLIC KEY-----";

    const JWK_JSON: &str = r#"{"use":"sig","alg":"RS256","kid":"1","kty":"RSA","n":"wmNDtlawWXevdB5fDT4gkmBl_al_Ij9KcyAZc78O2m3Cnb9nfcSlMuiChPP3RsehaoyXGl8dCV39gI0bEViAX5_OF5x39mFJdhe00-bUe4X7mKgItA6LN10PeIhJR59zis8URSzVDm7OsA2HLbrRxM5k0kapNBmFN1-Us2XnNn0V2_QR9iteQgkpe4I5_adAwj-fQo-dY13j5N11GP7jhnEmw7MiNjo0Kv9h1ZJJHDlb9edSmA6dg6jOxhlE4kQAJYObCF2TKhBcO-szl_zFVm9E55yiVYjWKk0R4uVXp9natMsQcbOLOAc5t3RZmw5L0Nikhso62g9oefgwWOIPJw","e":"AQAB"}"#;

    const JWKS_JSON: &str = r#"{"keys":[{"use":"sig","alg":"RS256","kid":"1","kty":"RSA","n":"wmNDtlawWXevdB5fDT4gkmBl_al_Ij9KcyAZc78O2m3Cnb9nfcSlMuiChPP3RsehaoyXGl8dCV39gI0bEViAX5_OF5x39mFJdhe00-bUe4X7mKgItA6LN10PeIhJR59zis8URSzVDm7OsA2HLbrRxM5k0kapNBmFN1-Us2XnNn0V2_QR9iteQgkpe4I5_adAwj-fQo-dY13j5N11GP7jhnEmw7MiNjo0Kv9h1ZJJHDlb9edSmA6dg6jOxhlE4kQAJYObCF2TKhBcO-szl_zFVm9E55yiVYjWKk0R4uVXp9natMsQcbOLOAc5t3RZmw5L0Nikhso62g9oefgwWOIPJw","e":"AQAB"}]}"#;

    const JWT: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJzdWIiOiIwMUdWRjRCQ00zMlRUTTE5WUJGQjJaTkU0UiIsImV4cCI6MTY3OTUzNTg5MSwiaWF0IjoxNjc4OTMxMDkxfQ.Uug8lF11bjBUYzEnNr-HIgwlkckVxtUh5RQyEAinmUzB4nZiMQEb9cTx9LsU0mLTLz7lHOdo49CmjyNqEnn-KDX9p0dn97AZISHihIjEtfBbbmh3DqsLBN_QzKyB_h4gLGT5Fx33tgYkx8Sbpk8GmIlidUsjrHqENPOoI3dZssdgi4z0TcZ7cSJgpnNy-fb63oHy0_Gmfu5viNGi8V4ydGR7hBFt6-SBr9aOF2Y6FeePmxKDdIz4PuOa0gQCGbiIdX1agx2SXnWAUYdnW4IlEUo_Yv-N-Rbf_YhAG0uENdcL6p-NfkMhey0stb9UMj8KVU6KKrP6TRn_TfO9jiF7qQ";

    /// Test that a jwks can be converted to a public key
    /// and that the public key can be used to verify a token
    #[tokio::test]
    async fn jwk_to_key() -> Result<()> {
        let jwk: jsonwebtoken::jwk::Jwk = serde_json::from_str(JWK_JSON)?;
        let key = PublicKey::try_from(jwk)?;

        // don't validate exp
        let mut validation = jsonwebtoken::Validation::new(Algorithm::RS256);
        validation.validate_exp = false;

        let key: jsonwebtoken::DecodingKey = axum::extract::FromRef::from_ref(&key);
        Claims::decode_validation(JWT, &key, &validation).await?;

        Ok(())
    }

    /// Test that a jwks can be converted to a public key
    /// and that the public key can be used to verify a token
    #[tokio::test]
    async fn jwks_to_key() -> Result<()> {
        let key_set = PublicKey::parse_from_jwks(JWKS_JSON)?;
        let key = key_set.first().unwrap();

        // don't validate exp
        let mut validation = jsonwebtoken::Validation::new(Algorithm::RS256);
        validation.validate_exp = false;

        let key: jsonwebtoken::DecodingKey = axum::extract::FromRef::from_ref(key);
        Claims::decode_validation(JWT, &key, &validation).await?;

        Ok(())
    }

    /// Test that an RSA PEM can be converted to a public key
    /// and that the public key can be used to verify a token
    #[tokio::test]
    async fn pem_to_key() -> Result<()> {
        let key = PublicKey::parse_from_pem(RSA_PUBLIC_KEY)?;

        let mut validation = jsonwebtoken::Validation::new(Algorithm::RS256);
        validation.validate_exp = false;

        let key: jsonwebtoken::DecodingKey = axum::extract::FromRef::from_ref(&key);
        Claims::decode_validation(JWT, &key, &validation).await?;

        Ok(())
    }
}
