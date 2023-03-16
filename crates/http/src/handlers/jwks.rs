use crate::{error::Result, ServerState};
use axum::extract::State;

// jwks handler
pub async fn jwks(
    State(ServerState { public_key, .. }): State<ServerState>,
) -> Result<axum::response::Json<jsonwebtoken::jwk::JwkSet>> {
    // Assume public_key is an RSA public key. It implements AsRef<[u8]> so we can use it to
    // construct a jsonwebtoken::jwk::Jwk for the public key. The value is bytes in PKCS#1 format.

    // extract the modulus and public exponent from the public key

    // build a jsonwebtoken::jwk::Jwk for the public key
    let jwk: jsonwebtoken::jwk::Jwk = public_key.into();

    let jwks = jsonwebtoken::jwk::JwkSet { keys: vec![jwk] };

    Ok(axum::response::Json(jwks))
}
