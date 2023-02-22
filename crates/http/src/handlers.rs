use crate::error::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use aws_sdk_dynamodb::model::AttributeValue;
use axum::Json;
use lockpad_models::user::{User, UserData};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod login;

pub(crate) async fn hello_world() -> axum::response::Html<&'static str> {
    axum::response::Html(
        r#"
        <html>
            <head>
                <title>Lockpad</title>
            </head>
            <body>
                <h1>Hello, World!</h1>
            </body>

            <a href="/login">Login</a>
        </html>
    "#,
    )
}

/// Sends a screen that asks the user to provide credentials.
pub(crate) async fn login_screen() -> axum::response::Html<&'static str> {
    // Keep this really simple for now.
    // Later this could be its own dedicated page, but for now it's just a simple form.

    axum::response::Html(
        r#"
        <h1>log in</h1>
        <form id="login-form">
            <input type="text" id="username" name="username" placeholder="username" />
            <input type="password" id="password" name="password" placeholder="password" />
            <input type="submit" value="Login" />
        </form>

        <script>
            const form = document.getElementById("login-form");
            form.onsubmit = function(event) {
                console.log("submitting form");
                event.preventDefault();
                const data = new FormData(form);
                const username = data.get("username");
                const password = data.get("password");

                // Perform a POST request to /authorize
                // If the request is successful, the s
                //
                fetch("/authorize", {
                    method: "POST",
                    body: JSON.stringify({ username, password }),
                    headers: {
                        "Content-Type": "application/json",
                    },
                })
                .then(response => response.json())
                .then(data => {
                    console.log("Success:", data);
                    // navigate to the main page
                    // TODO: this value should be populated from the server
                    window.location.href = "/";
                })
                .catch((error) => {
                    console.error("Error:", error);
                });
            }
        </script>

        <style>
            form {
                display: flex;
                flex-direction: column;
                align-items: center;
            }

            input {
                margin: 0.5rem;
            }

            input[type="submit"] {
                width: 100px;
            }

            input[type="text"], input[type="password"] {
                width: 200px;
            }

            input[type="text"]:focus, input[type="password"]:focus {
                outline: none;
            }

            h1 {
                text-align: center;
            }
        </style>
    "#,
    )
}

#[derive(Debug, Deserialize)]
pub(crate) struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct AuthorizeResponse {
    token: String,
}

/// Performs the authorization process.
/// This is where the user's credentials are checked against the database.
/// If the credentials are valid, a token is generated and sent to the user.
pub(crate) async fn authorize(
    payload: axum::extract::Json<Credentials>,
) -> axum::response::Json<AuthorizeResponse> {
    // TODO: Implement authorization
    // For now, just print the credentials to the console.
    let credentials = payload.0;
    tracing::info!(?credentials.username, ?credentials.password, "Received credentials");

    // for now, return a dummy token
    axum::response::Json(AuthorizeResponse {
        token: "dummy".to_string(),
    })
}

/// Sends a screen that asks the user to provide credentials.
/// These credentials will be used to create a new account.
/// This closely follows the login screen.
pub(crate) async fn signup_screen() -> axum::response::Html<&'static str> {
    // Keep this really simple for now.
    // Later this could be its own dedicated page, but for now it's just a simple form.

    axum::response::Html(
        r#"
        <h1>sign up</h1>
        <form id="signup-form">
            <input type="text" id="username" name="username" placeholder="username" />
            <input type="password" id="password" name="password" placeholder="password" />
            <input type="submit" value="Sign up" />
        </form>

        <script>
            const form = document.getElementById("signup-form");
            form.onsubmit = function(event) {
                console.log("submitting form");
                event.preventDefault();
                const data = new FormData(form);
                const username = data.get("username");
                const password = data.get("password");

                // Perform a POST request to /signup
                // If the request is successful, the s
                //
                fetch("/signup", {
                    method: "POST",
                    body: JSON.stringify({ username, password }),
                    headers: {
                        "Content-Type": "application/json",
                    },
                })
                .then(response => response.json())
                .then(data => {
                    console.log("Success:", data);
                    // navigate to the main page
                    window.location.href = "/";
                })
                .catch((error) => {
                    console.error("Error:", error);
                });

            }
        </script>

        <style>
            form {
                display: flex;
                flex-direction: column;
                align-items: center;
            }

            input {
                margin: 0.5rem;
            }

            input[type="submit"] {
                width: 100px;
            }

            input[type="text"], input[type="password"] {
                width: 200px;
            }

            input[type="text"]:focus, input[type="password"]:focus {
                outline: none;
            }

            h1 {
                text-align: center;
            }
        </style>
    "#,
    )
}

pub(crate) async fn list_users(
    dynamodb: axum::extract::State<scylla_dynamodb::DynamodbTable>,
) -> Result<Json<Vec<lockpad_models::user::User>>> {
    let client = &dynamodb.client;

    let res = client
        .query()
        .table_name(&dynamodb.name)
        .key_condition_expression("#pk = :pk")
        .expression_attribute_names("#pk", "pk")
        .expression_attribute_values(
            ":pk",
            AttributeValue::S(lockpad_models::user::User::PREFIX.to_string()),
        )
        .send()
        .await?;

    tracing::info!(?res, "query result");

    let items = res.items().map(|slice| slice.to_vec()).unwrap();

    let users = items
        .into_iter()
        .map(|item| {
            let user: lockpad_models::user::User = serde_dynamo::from_item(item).unwrap();
            tracing::info!(?user, "user");
            user
        })
        .collect::<Vec<_>>();

    Ok(Json(users))
}

/// Performs the signup process.
/// This is where the user's credentials are checked against the database.
/// If the credentials are valid, a token is generated and sent to the user.
pub(crate) async fn signup(
    dynamodb: axum::extract::State<scylla_dynamodb::DynamodbTable>,
    payload: axum::extract::Json<Credentials>,
) -> Result<axum::response::Json<AuthorizeResponse>> {
    // TODO: Check against database to see if the username is already taken.

    // TODO: Hash the password before storing it in the database.
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password = payload.0.password.into_bytes();
    let password_hash = argon2.hash_password(&password, &salt).unwrap().to_string();

    let user = User::new(
        salt.to_string(),
        UserData {
            identifier: payload.0.username,
            secret: password_hash,
        },
    );

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
