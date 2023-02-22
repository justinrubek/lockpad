use crate::error::{Error, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use aws_sdk_dynamodb::model::AttributeValue;
use lockpad_models::user::{User, UserData};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub(crate) struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct AuthorizeResponse {
    token: String,
}

/// Performs the signup process.
/// This is where the user's credentials are added to the database.
/// If the credentials are unique, the acount is created and a token is sent to the user.
pub(crate) async fn signup(
    dynamodb: axum::extract::State<scylla_dynamodb::DynamodbTable>,
    payload: axum::extract::Json<Credentials>,
) -> Result<axum::response::Json<AuthorizeResponse>> {
    // TODO: Check against database to see if the username is already taken.

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password = payload.0.password.into_bytes();
    let password_hash = argon2.hash_password(&password, &salt).unwrap().to_string();

    let user = User::new(UserData {
        identifier: payload.0.username,
        secret: password_hash,
    });

    let client = &dynamodb.client;

    // We need to prepare the pk and sk attributes for the user manually
    let mut item_data: HashMap<String, AttributeValue> = serde_dynamo::to_item(&user).unwrap();
    item_data.insert(
        "pk".to_string(),
        AttributeValue::S(User::PREFIX.to_string()),
    );
    item_data.insert(
        "sk".to_string(),
        AttributeValue::S(format!("{}#{}", User::PREFIX, user.data.identifier)),
    );
    tracing::info!(?item_data, "item being created");

    let res = client
        .put_item()
        .table_name(&dynamodb.name)
        .set_item(Some(item_data))
        .send()
        .await?;

    tracing::info!(?res, "put item result");

    // for now, return a dummy token
    Ok(axum::response::Json(AuthorizeResponse {
        token: "dummy".to_string(),
    }))
}

/// Performs the authorization process.
/// This is where the user's credentials are checked against the database.
/// If the credentials are valid, a token is generated and sent to the user.
pub(crate) async fn authorize(
    dynamodb: axum::extract::State<scylla_dynamodb::DynamodbTable>,
    payload: axum::extract::Json<Credentials>,
) -> Result<axum::response::Json<AuthorizeResponse>> {
    let input_credentials = payload.0;

    let client = &dynamodb.client;
    // Query to find the user with the given username
    let res = client
        .query()
        .table_name(&dynamodb.name)
        .key_condition_expression("#pk = :pk AND #sk = :sk")
        .expression_attribute_names("#pk", "pk")
        .expression_attribute_names("#sk", "sk")
        .expression_attribute_values(":pk", AttributeValue::S(User::PREFIX.to_string()))
        .expression_attribute_values(
            ":sk",
            AttributeValue::S(format!("{}#{}", User::PREFIX, input_credentials.username)),
        )
        .send()
        .await?;

    match res.count() {
        0 => {
            tracing::debug!("No user found with the given username");
            Err(Error::Unauthorized)
        }
        1 => {
            let user = res.items().unwrap()[0].clone();
            let user: User = serde_dynamo::from_item(user).unwrap();
            tracing::debug!(?user, "user found");

            let password_hash = PasswordHash::new(&user.data.secret).unwrap();
            Argon2::default()
                .verify_password(input_credentials.password.as_bytes(), &password_hash)
                .map_err(|_| {
                    tracing::debug!("Password verification failed");
                    Error::Unauthorized
                })?;

            tracing::debug!("password verified");

            // for now, return a dummy token
            Ok(axum::response::Json(AuthorizeResponse {
                token: "dummy".to_string(),
            }))
        }
        _ => {
            // should not be possible
            tracing::error!("Multiple users found with the same username");
            Err(Error::Unauthorized)
        }
    }
}
