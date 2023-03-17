use crate::{error::Result, ServerState};
use axum::extract::State;

// jwks handler
pub async fn jwks(
    State(ServerState { public_key, .. }): State<ServerState>,
) -> Result<axum::response::Json<jsonwebtoken::jwk::JwkSet>> {
    let jwk: jsonwebtoken::jwk::Jwk = public_key.into();
    let jwks = jsonwebtoken::jwk::JwkSet { keys: vec![jwk] };

    Ok(axum::response::Json(jwks))
}
