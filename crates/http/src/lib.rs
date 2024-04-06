use axum::{
    extract::FromRef,
    routing::{get, post},
    Router,
};
use lockpad_auth::PublicKey;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub mod error;
pub mod handlers;
pub mod validation;

use error::Result;
use handlers::{
    auth::{authorize, authorize_json, register},
    pages::{disabled_register_screen, login_screen, register_screen, root},
    user::{get_user, list_users},
};

pub struct Server {
    addr: SocketAddr,

    pg_pool: sqlx::pool::Pool<sqlx::Postgres>,

    /// The secret used to sign the JWT tokens.
    jwt_secret: Vec<u8>,
    /// The public key used to verify the JWT tokens.
    jwt_public: Vec<u8>,

    disable_signup: bool,
}

#[derive(Clone)]
pub struct ServerState {
    pub pg_pool: sqlx::pool::Pool<sqlx::Postgres>,
    pub encoding_key: jsonwebtoken::EncodingKey,
    pub public_key: PublicKey,
}

impl FromRef<ServerState> for PublicKey {
    fn from_ref(state: &ServerState) -> Self {
        state.public_key.clone()
    }
}

impl Server {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub async fn run(self) -> Result<()> {
        let cors = tower_http::cors::CorsLayer::permissive();

        let encoding_key = jsonwebtoken::EncodingKey::from_rsa_pem(&self.jwt_secret)?;
        let public_key = PublicKey::new(self.jwt_public)?;
        let state = ServerState {
            pg_pool: self.pg_pool,
            encoding_key,
            public_key,
        };

        let mut app = Router::new()
            .route("/", get(root))
            .route("/login", get(login_screen))
            .route("/forms/authorize", post(authorize))
            .route("/api/authorize", post(authorize_json))
            .route("/users", get(list_users))
            .route("/users/:user_id", get(get_user))
            .route(
                "/applications",
                get(handlers::application::list_applications)
                    .post(handlers::application::create_application),
            )
            .route(
                "/applications/:application_id",
                get(handlers::application::get_application),
            )
            .route(
                "/api-keys",
                get(handlers::api_key::list_api_keys).post(handlers::api_key::create_api_key),
            )
            .route("/api-keys/:api_key_id", get(handlers::api_key::get_api_key))
            .route("/.well-known/jwks.json", get(handlers::jwks::jwks))
            .route("/health", get(handlers::health::health));
        if !self.disable_signup {
            app = app
                .route("/forms/register", post(register))
                .route("/register", get(register_screen));
        } else {
            app = app.route("/register", get(disabled_register_screen));
        }
        let app = app.with_state(state).layer(cors);

        tracing::info!("Listening on {0}", self.addr);
        let listener = TcpListener::bind(&self.addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}

pub struct Builder {
    addr: Option<SocketAddr>,
    pg_pool: Option<sqlx::pool::Pool<sqlx::Postgres>>,
    jwt_secret: Option<Vec<u8>>,
    jwt_public: Option<Vec<u8>>,
    disable_signup: Option<bool>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            addr: None,
            pg_pool: None,
            jwt_secret: None,
            jwt_public: None,
            disable_signup: None,
        }
    }

    pub fn addr(mut self, addr: SocketAddr) -> Self {
        self.addr = Some(addr);
        self
    }

    pub fn pg_pool(mut self, pg_pool: sqlx::pool::Pool<sqlx::Postgres>) -> Self {
        self.pg_pool = Some(pg_pool);
        self
    }

    pub fn jwt_secret(mut self, jwt_secret: Vec<u8>) -> Self {
        self.jwt_secret = Some(jwt_secret);
        self
    }

    pub fn jwt_public(mut self, jwt_public: Vec<u8>) -> Self {
        self.jwt_public = Some(jwt_public);
        self
    }

    pub fn disable_signup(mut self, disable_signup: bool) -> Self {
        self.disable_signup = Some(disable_signup);
        self
    }

    pub fn build(self) -> Result<Server> {
        let addr = self.addr.ok_or(error::Error::ServerBuilder)?;
        let pg_pool = self.pg_pool.ok_or(error::Error::ServerBuilder)?;
        let jwt_secret = self.jwt_secret.ok_or(error::Error::ServerBuilder)?;
        let jwt_public = self.jwt_public.ok_or(error::Error::ServerBuilder)?;
        let disable_signup = self.disable_signup.unwrap_or(false);

        Ok(Server {
            addr,
            pg_pool,
            jwt_secret,
            jwt_public,
            disable_signup,
        })
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            addr: Some(SocketAddr::from(([0, 0, 0, 0], 5000))),
            pg_pool: None,
            jwt_secret: None,
            jwt_public: None,
            disable_signup: None,
        }
    }
}
