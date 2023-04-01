use std::str::FromStr;

use crate::{
    error::{Error, Result},
    ServerState,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::extract::State;
use jsonwebtoken::EncodingKey;
use lockpad_auth::Claims;
use lockpad_models::{api_key::ApiKey, entity::Builder, user::User};
use lockpad_ulid::Ulid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub(crate) struct UserCredentials {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ApiKeyCredentials {
    api_key_id: String,
    api_secret: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum Credentials {
    User(UserCredentials),
    ApiKey(ApiKeyCredentials),
}

#[derive(Debug, Serialize)]
pub(crate) struct AuthorizeResponse {
    token: String,
}

/// Hashes a string using argon2.
/// This is  performed on any password before it is stored in the database.
pub(crate) async fn hash_string(data: &[u8]) -> Result<String> {
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
    State(ServerState {
        encoding_key,
        pg_pool,
        ..
    }): State<ServerState>,
    payload: axum::extract::Json<UserCredentials>,
) -> Result<axum::response::Json<AuthorizeResponse>> {
    // TODO: Check against database to see if the username is already taken.

    let password_hash = hash_string(&payload.0.password.into_bytes()).await?;

    let user = User::builder()
        .identifier(payload.0.username)
        .secret(password_hash)
        .build()?;

    tracing::debug!(?user, "creating user");
    user.create(&pg_pool).await?;

    let user_id = user.user_id.to_string();
    let token = Claims::new(user_id).encode(&encoding_key).await?;

    // for now, return a dummy token
    Ok(axum::response::Json(AuthorizeResponse { token }))
}

/// Performs the authorization process.
/// This is where the user's credentials are checked against the database.
/// If the credentials are valid, a token is generated and sent to the user.
pub(crate) async fn authorize(
    State(ServerState {
        encoding_key,
        pg_pool,
        ..
    }): State<ServerState>,
    payload: axum::extract::Json<Credentials>,
) -> Result<axum::response::Json<AuthorizeResponse>> {
    match payload.0 {
        Credentials::User(payload) => authorize_user(payload, &encoding_key, &pg_pool).await,
        Credentials::ApiKey(payload) => authorize_api_key(payload, &encoding_key, &pg_pool).await,
    }
}

async fn authorize_user(
    payload: UserCredentials,
    encoding_key: &EncodingKey,
    pg_pool: &sqlx::PgPool,
) -> Result<axum::response::Json<AuthorizeResponse>> {
    let user = User::by_identifier(pg_pool, &payload.username).await?;

    match user {
        None => {
            tracing::debug!("user not found");

            Err(Error::Unauthorized)
        }
        Some(user) => {
            tracing::debug!(?user.user_id, "user found");
            tracing::debug!(?user.user_id, "user found");

            validate_hash(payload.password.as_bytes(), &user.secret).await?;
            tracing::debug!("password verified");

            let token = Claims::new(user.user_id.to_string())
                .encode(encoding_key)
                .await?;
            Ok(axum::response::Json(AuthorizeResponse { token }))
        }
    }
}

async fn authorize_api_key(
    payload: ApiKeyCredentials,
    encoding_key: &EncodingKey,
    pg_pool: &sqlx::PgPool,
) -> Result<axum::response::Json<AuthorizeResponse>> {
    let api_key = Ulid::from_str(&payload.api_key_id).map_err(|_| Error::Unauthorized)?;
    let api_key = ApiKey::by_id(pg_pool, &api_key).await?;

    match api_key {
        None => {
            tracing::debug!("user not found");

            Err(Error::Unauthorized)
        }
        Some(api_key) => {
            let owner_id = api_key.owner_id.to_string();

            tracing::debug!(owner_id, "user found");

            validate_hash(payload.api_secret.as_bytes(), &api_key.secret).await?;

            let token = Claims::new(owner_id.to_string())
                .encode(encoding_key)
                .await?;
            Ok(axum::response::Json(AuthorizeResponse { token }))
        }
    }
}
