use crate::error::Result;
use axum::extract::State;
use lockpad_auth::PublicKey;

// jwks handler
pub async fn jwks(
    State(public_key): State<PublicKey>,
) -> Result<axum::response::Json<jsonwebtoken::jwk::JwkSet>> {
    let jwk: jsonwebtoken::jwk::Jwk = public_key.into();
    let jwks = jsonwebtoken::jwk::JwkSet { keys: vec![jwk] };

    Ok(axum::response::Json(jwks))
}
