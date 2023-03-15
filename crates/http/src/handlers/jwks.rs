use crate::{error::Result, ServerState};
use axum::extract::State;
use base64::Engine;

#[derive(Clone)]
pub struct PublicKey(pub Vec<u8>);

// jwks handler
pub async fn jwks(
    State(ServerState {
        public_key: PublicKey(key),
        ..
    }): State<ServerState>,
) -> Result<axum::response::Json<serde_json::Value>> {
    // Manually construct the JSON response
    let mut json = serde_json::Map::new();

    let engine = base64::engine::general_purpose::STANDARD;
    let encoded_key = engine.encode(key);

    // TODO: support multiple keys
    let mut key = serde_json::Map::new();
    key.insert("kty".to_string(), "OKP".to_string().into());
    key.insert("crv".to_string(), "Ed25519".to_string().into());
    key.insert("alg".to_string(), "EdDSA".to_string().into());
    key.insert("x".to_string(), encoded_key.into());
    key.insert("kid".to_string(), "1".to_string().into());

    json.insert("keys".to_string(), vec![key].into());
    let json = serde_json::Value::Object(json);

    Ok(axum::response::Json(json))
}
