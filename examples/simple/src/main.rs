use axum::{
    routing::{get, post},
    Router,
};
use lockpad_auth::PublicKey;
use lockpad_http::error::Result;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
    let auth_url = std::env::var("AUTH_URL").unwrap_or_else(|_| panic!("AUTH_URL must be set"));

    // load the public key from the auth server
    let client = reqwest::Client::new();
    let res = client
        .get(format!("{auth_url}/.well-known/jwks.json"))
        .send()
        .await
        .unwrap();
    let jwks_str = res.text().await.unwrap();

    let key_set = PublicKey::parse_from_jwks(&jwks_str)?;
    let public_key = key_set[0].clone();

    let state = ServerState { public_key };

    let app = Router::new()
        .route("/unprotected", get(unprotected))
        .route("/protected", post(protected_claims))
        .with_state(state)
        .layer(tower_http::cors::CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn unprotected() -> &'static str {
    "Unprotected"
}

async fn protected_claims(claims: lockpad_auth::Claims) -> String {
    format!("Hello, {}!", claims.sub)
}

#[derive(Clone)]
struct ServerState {
    public_key: PublicKey,
}

/// This is needed for the implementation of [FromRequestParts](axum::extract::FromRequestParts) on [Claims](lockpad_auth::Claims)
impl axum::extract::FromRef<ServerState> for PublicKey {
    fn from_ref(state: &ServerState) -> Self {
        state.public_key.clone()
    }
}
