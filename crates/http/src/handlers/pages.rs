use std::str::FromStr;

use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use lockpad_models::application::Application;
use lockpad_ulid::Ulid;
use serde::Deserialize;

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

    HtmlPage::Form(HtmlForm::login("/authorize".to_string()))
}

/// Sends a screen that asks the user to provide credentials.
/// These credentials will be used to create a new account.
/// This closely follows the login screen.
pub(crate) async fn signup_screen() -> impl IntoResponse {
    // Keep this really simple for now.
    // Later this could be its own dedicated page, but for now it's just a simple form.
    HtmlPage::Form(HtmlForm::register("/signup".to_string()))
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
    Form(HtmlForm),
    /// The default page
    Default,
}

impl axum::response::IntoResponse for HtmlPage {
    fn into_response(self) -> axum::response::Response {
        let common_html = r#"
        <style>
            .container {
                display: flex;
                flex-direction: column;
                align-items: center;
            }

            h1 {
                text-align: center;
            }

            p {
                width: 500px;
            }

            ul {
                width: 500px;
            }

            li {
                margin: 0.5rem;
            }

            li strong {
                font-weight: bold;
            }

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
        </style>
    "#;

        let html = match self {
            HtmlPage::Form(form) => form.to_string(),
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

enum HtmlFormType {
    /// Allow for user account creation
    Register,
    /// Allows for user login
    Login,
}

/// Represents the html for <form> + <script> tags
struct HtmlForm {
    form_type: HtmlFormType,
    submit_uri: String,
}

impl HtmlForm {
    fn new(form_type: HtmlFormType, submit_uri: String) -> Self {
        Self {
            form_type,
            submit_uri,
        }
    }

    /// Build a registeration form
    fn register(submit_uri: String) -> Self {
        Self::new(HtmlFormType::Register, submit_uri)
    }

    /// Build a login form
    fn login(submit_uri: String) -> Self {
        Self::new(HtmlFormType::Login, submit_uri)
    }
}

impl std::string::ToString for HtmlForm {
    fn to_string(&self) -> String {
        let type_name = match self.form_type {
            HtmlFormType::Register => "register",
            HtmlFormType::Login => "login",
        };

        let type_display = match self.form_type {
            HtmlFormType::Register => "Sign up",
            HtmlFormType::Login => "Login",
        };

        let submit_uri = &self.submit_uri;
        let form_name = format!("{}-form", type_name);

        let form_html = format!(
            r#"
            <form 
                id="{form_name}"
                action="{submit_uri}"
                method="POST"
            >
                <input type="text" id="username" name="username" placeholder="username" />
                <input type="password" id="password" name="password" placeholder="password" />
                <input type="submit" value="{type_display}" />
            </form>
        "#,
        );

        format!(
            r#"
                <h1>{type_display}</h1>
                {form_html}
            "#,
        )
    }
}
