use crate::{
    error::{Error, Result},
    ServerState,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::extract::State;
use lockpad_auth::Claims;
use lockpad_models::{entity::Builder, user::User};
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
    State(ServerState {
        encoding_key,
        pg_pool,
        ..
    }): State<ServerState>,
    payload: axum::extract::Json<Credentials>,
) -> Result<axum::response::Json<AuthorizeResponse>> {
    // TODO: Check against database to see if the username is already taken.

    let password_hash = hash_string(&payload.0.password.into_bytes()).await?;

    let user = User::builder()
        .identifier(payload.0.username)
        .secret(password_hash)
        .build()?;

    tracing::info!(?user, "creating user");
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
    let user = User::by_identifier(&pg_pool, &payload.0.username).await?;

    match user {
        None => {
            tracing::debug!("user not found");

            Err(Error::Unauthorized)
        }
        Some(user) => {
            tracing::debug!(?user.user_id, "user found");
            tracing::debug!(?user.user_id, "user found");

            validate_hash(payload.0.password.as_bytes(), &user.secret).await?;
            tracing::debug!("password verified");

            let token = Claims::new(user.user_id.to_string())
                .encode(&encoding_key)
                .await?;
            Ok(axum::response::Json(AuthorizeResponse { token }))
        }
    }
}
