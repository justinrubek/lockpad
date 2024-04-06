use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use dioxus::prelude::*;
use lockpad_models::application::Application;
use lockpad_ulid::Ulid;
use serde::Deserialize;
use std::str::FromStr;

use crate::ServerState;

pub(crate) async fn root() -> impl IntoResponse {
    HtmlPage::Default
}

#[derive(Debug, Deserialize)]
pub(crate) struct LoginScreenQuery {
    pub redirect_uri: String,
    pub client_id: String,
}

/// Sends a screen that asks the user to provide credentials.
pub(crate) async fn login_screen(
    query: Option<Query<LoginScreenQuery>>,
    State(ServerState { pg_pool, .. }): State<ServerState>,
) -> impl IntoResponse {
    let params = match query {
        None => return HtmlPage::NoParams,
        Some(query) => query.0,
    };
    tracing::debug!("login screen query: {:?}", params);

    // Lookup the client_id as the application_id and determine if the redirect_uri and origins are valid.
    // If the client_id is not valid, return an error.
    // If the redirect_uri is not valid, return an error.
    let app_id = Ulid::from_str(&params.client_id).unwrap();
    let application = match Application::by_id(&pg_pool, &app_id).await {
        Err(_) => return HtmlPage::InvalidParams,
        Ok(None) => return HtmlPage::InvalidParams,
        Ok(Some(application)) => application,
    };
    tracing::debug!("application: {:?}", application);

    // compare the redirect_uri to application.redirect_uris
    if !application
        .allowed_callback_urls
        .contains(&params.redirect_uri)
    {
        return HtmlPage::InvalidParams;
    }

    // TODO: compare the origin to application.allowed_origins

    HtmlPage::CredentialsForm {
        form_type: HtmlFormType::Login,
        submit_uri: "/login".to_string(),
    }
}

/// Sends a screen that asks the user to provide credentials.
/// These credentials will be used to create a new account.
/// This closely follows the login screen.
pub(crate) async fn signup_screen() -> impl IntoResponse {
    // Keep this really simple for now.
    // Later this could be its own dedicated page, but for now it's just a simple form.
    HtmlPage::CredentialsForm {
        form_type: HtmlFormType::Register,
        submit_uri: "/signup".to_string(),
    }
}

/// A response that sends an HTML page
enum HtmlPage {
    /// There was no origin provided in the request.
    #[allow(dead_code)]
    NoOrigin,
    /// There were no application parameters provided in the request.
    NoParams,
    /// The application parameters were invalid.
    InvalidParams,
    /// display a form
    CredentialsForm {
        form_type: HtmlFormType,
        submit_uri: String,
    },
    /// The default page
    Default,
}

const STYLE: &str = include_str!("style.css");

impl axum::response::IntoResponse for HtmlPage {
    fn into_response(self) -> axum::response::Response {
        let common_html = format!("<style>{STYLE}</style>");

        let html = match self {
            HtmlPage::CredentialsForm {
                form_type,
                submit_uri,
            } => dioxus_ssr::render_element(rsx!(
                login_form {
                    form_type: form_type,
                    submit_uri: submit_uri,
                }
            )),
            HtmlPage::NoOrigin => {
                r#"
                <div class="container">
                    <h1>log in</h1>
                    <p>
                        If you are seeing this page, it means you have arrived here by accident.
                        Your request did not include an origin header, which is required for security reasons.
                    </p>
                </div>
            "#.to_string()
            }
            HtmlPage::NoParams => {
                r#"
                <div class="container">
                    <h1>log in</h1>
                    <p>
                    If you are seeing this page, it means that you have been directed here by an application that is trying to log you in.
                    However, the application did not provide the necessary parameters to complete the login process.
                    The application needs to provide the following parameters:
                    </p>
                    <ul>
                        <li><strong>redirect_uri</strong></li>
                        <li><strong>client_id</strong></li>
                    </ul>
                    <p>
                    Without this information, the login page has no way of knowing where to redirect you after you have logged in.
                    </p>
                </div>
            "#.to_string()
            }
            HtmlPage::InvalidParams => {
                r#"
                <div class="container">
                    <h1>log in</h1>
                    <p>
                    If you are seeing this page, it means that you have been directed here by an application that is trying to log you in.
                    However, the application did not provide the necessary parameters to complete the login process.
                    The application needs to provide the following parameters:
                    </p>
                    <ul>
                        <li><strong>redirect_uri</strong></li>
                        <li><strong>client_id</strong></li>
                    </ul>
                    <p>
                    Without this information, the login page has no way of knowing where to redirect you after you have logged in.
                    </p>
                </div>
            "#.to_string()
            }
            HtmlPage::Default => {
                r#"
                    <div class="container">
                        <h1>lockpad</h1>
                        <a href="/login">log in</a>
                        <a href="/signup">sign up</a>
                    </div>
                "#.to_string()
            }
        };

        axum::response::Html(format!("{}{}", common_html, html)).into_response()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HtmlFormType {
    /// Allow for user account creation
    Register,
    /// Allows for user login
    Login,
}

#[component]
fn login_form(form_type: HtmlFormType, submit_uri: String) -> Element {
    let form_name = match form_type {
        HtmlFormType::Register => "register-form",
        HtmlFormType::Login => "login-form",
    };
    let type_display = match form_type {
        HtmlFormType::Register => "Sign up",
        HtmlFormType::Login => "Login",
    };

    rsx!(
        h1 { {type_display} }
        form {
            id: form_name,
            action: submit_uri,
            method: "POST",
            input {
                r#type: "text",
                id: "username",
                name: "username",
                placeholder: "username",
            }
            input {
                r#type: "password",
                id: "password",
                name: "password",
                placeholder: "password",
            }
            input {
                r#type: "submit",
                value: type_display,
            }
        }
    )
}
