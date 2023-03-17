use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use lockpad_auth::{Claims, PublicKey};
use lockpad_http::error::Result;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
    let auth_url = std::env::var("AUTH_URL").unwrap_or_else(|_| panic!("AUTH_URL must be set"));

    // load the public key from the auth server
    let client = reqwest::Client::new();
    let res = client
        .get(format!("{}/.well-known/jwks.json", auth_url))
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
        .route("/protected-user", post(protected_user))
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

// You can use the Claims type directly
async fn protected_claims(claims: lockpad_auth::Claims) -> String {
    format!("Hello, {}!", claims.sub)
}

// Or, you can wrap it by implementing FromRequestParts for your own type
async fn protected_user(user: User) -> String {
    format!("Hello, {}!", user.id)
}

struct User {
    id: String,
}

// This implementation of FromRequestParts will extract the claims and wrap it in a User
#[axum::async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync + AsRef<PublicKey>,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        // Extract the authorization header
        let claims = <Claims>::from_request_parts(parts, state)
            .await
            .map_err(|err| err.into_response())?;

        let user = User { id: claims.sub };

        Ok(user)
    }
}

#[derive(Clone)]
struct ServerState {
    public_key: PublicKey,
}

impl AsRef<PublicKey> for ServerState {
    fn as_ref(&self) -> &PublicKey {
        &self.public_key
    }
}
