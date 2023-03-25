use axum::{
    routing::{get, post},
    Router,
};
use lockpad_auth::PublicKey;
use std::net::SocketAddr;

pub mod error;
pub mod handlers;

use error::Result;
use handlers::{
    auth::{authorize, register},
    pages::{login_screen, root, signup_screen},
    user::{get_user, list_users},
};

pub struct Server {
    addr: SocketAddr,

    pg_pool: sqlx::pool::Pool<sqlx::Postgres>,

    /// The secret used to sign the JWT tokens.
    jwt_secret: Vec<u8>,
    /// The public key used to verify the JWT tokens.
    jwt_public: Vec<u8>,
}

#[derive(Clone)]
pub struct ServerState {
    pub pg_pool: sqlx::pool::Pool<sqlx::Postgres>,
    pub encoding_key: jsonwebtoken::EncodingKey,
    pub public_key: PublicKey,
}

impl AsRef<PublicKey> for ServerState {
    fn as_ref(&self) -> &PublicKey {
        &self.public_key
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

        let app = Router::new()
            .route("/", get(root))
            .route("/login", get(login_screen))
            .route("/signup-screen", get(signup_screen))
            .route("/authorize", post(authorize))
            .route("/signup", post(register))
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
            .route("/.well-known/jwks.json", get(handlers::jwks::jwks))
            .with_state(state)
            .layer(cors);

        tracing::info!("Listening on {0}", self.addr);
        axum::Server::bind(&self.addr)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}

pub struct Builder {
    addr: Option<SocketAddr>,
    pg_pool: Option<sqlx::pool::Pool<sqlx::Postgres>>,
    jwt_secret: Option<Vec<u8>>,
    jwt_public: Option<Vec<u8>>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            addr: None,
            pg_pool: None,
            jwt_secret: None,
            jwt_public: None,
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

    pub fn build(self) -> Result<Server> {
        let addr = self.addr.ok_or(error::Error::ServerBuilder)?;
        let pg_pool = self.pg_pool.ok_or(error::Error::ServerBuilder)?;
        let jwt_secret = self.jwt_secret.ok_or(error::Error::ServerBuilder)?;
        let jwt_public = self.jwt_public.ok_or(error::Error::ServerBuilder)?;

        Ok(Server {
            addr,
            pg_pool,
            jwt_secret,
            jwt_public,
        })
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            addr: Some(SocketAddr::from(([0, 0, 0, 0], 3000))),
            pg_pool: None,
            jwt_secret: None,
            jwt_public: None,
        }
    }
}
