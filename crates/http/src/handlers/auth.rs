use crate::{
    error::{Error, Result},
    ServerState,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::extract::State;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use lockpad_models::{entity::Builder, user::User};
use scylla_dynamodb::entity::{FormatKey, GetEntity, PutEntity};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub(crate) struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct AuthorizeResponse {
    token: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

fn generate_claims(sub: String) -> Result<Claims> {
    let now = std::time::SystemTime::now();
    let iat = now.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as usize;

    let token_life = std::time::Duration::from_secs(60 * 60 * 24 * 7); // 7 days
    let exp = (now + token_life)
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() as usize;

    Ok(Claims { sub, exp, iat })
}

async fn generate_token(sub: String) -> Result<String> {
    let priv_key = tokio::fs::read("keypair.der").await?;
    let key = EncodingKey::from_ed_pem(&priv_key)?;

    let claims = generate_claims(sub)?;
    let header = Header::new(Algorithm::EdDSA);
    let token = encode(&header, &claims, &key)?;

    Ok(token)
}

/// Hashes a string using argon2.
/// This is  performed on any password before it is stored in the database.
async fn hash_string(data: &[u8]) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(data, &salt)?.to_string();

    Ok(password_hash)
}

async fn validate_hash(data: &[u8], secret: &str) -> Result<()> {
    let password_hash = PasswordHash::new(secret).unwrap();
    Argon2::default()
        .verify_password(data, &password_hash)
        .map_err(|_| {
            tracing::debug!("Password verification failed");
            Error::Unauthorized
        })?;

    Ok(())
}

/// Performs the signup process.
/// This is where the user's credentials are added to the database.
/// If the credentials are unique, the acount is created and a token is sent to the user.
pub(crate) async fn register(
    State(ServerState { dynamodb, .. }): State<ServerState>,
    payload: axum::extract::Json<Credentials>,
) -> Result<axum::response::Json<AuthorizeResponse>> {
    // TODO: Check against database to see if the username is already taken.

    let password_hash = hash_string(&payload.0.password.into_bytes()).await?;

    let user = User::builder()
        .identifier(payload.0.username)
        .secret(password_hash)
        .build()?;

    tracing::info!(?user, "creating user");
    let res = user.put_item(&dynamodb)?.send().await?;

    tracing::info!(?res, "put item result");

    let user_id = user.id.to_string();
    let token = generate_token(user_id).await?;

    // for now, return a dummy token
    Ok(axum::response::Json(AuthorizeResponse { token }))
}

/// Performs the authorization process.
/// This is where the user's credentials are checked against the database.
/// If the credentials are valid, a token is generated and sent to the user.
pub(crate) async fn authorize(
    State(ServerState { dynamodb, .. }): State<ServerState>,
    payload: axum::extract::Json<Credentials>,
) -> Result<axum::response::Json<AuthorizeResponse>> {
    let key = User::format_key(payload.0.username);
    let res = User::get(&dynamodb, key)?.send().await?;

    match res.item() {
        None => {
            tracing::debug!("No user found with the given username");
            Err(Error::Unauthorized)
        }
        Some(item) => {
            let user: User = serde_dynamo::from_item(item.to_owned())?;
            tracing::debug!(?user.id, "user found");

            validate_hash(payload.0.password.as_bytes(), &user.secret).await?;
            tracing::debug!("password verified");

            let token = generate_token(user.id.to_string()).await?;
            Ok(axum::response::Json(AuthorizeResponse { token }))
        }
    }
}
