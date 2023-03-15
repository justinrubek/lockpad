use axum::{
    routing::{get, post},
    Router,
};
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
    client: aws_sdk_dynamodb::Client,
    table_name: String,

    /// The secret used to sign the JWT tokens.
    jwt_secret: Vec<u8>,
    /// The public key used to verify the JWT tokens.
    jwt_public: Vec<u8>,
}

#[derive(Clone)]
pub struct ServerState {
    pub dynamodb: scylla_dynamodb::DynamodbTable,
    pub encoding_key: jsonwebtoken::EncodingKey,
    pub public_key: handlers::jwks::PublicKey,
}

impl Server {
    pub fn builder() -> Builder {
        Builder::new()
    }

    pub async fn run(self) -> Result<()> {
        let cors = tower_http::cors::CorsLayer::permissive();

        let dynamodb = scylla_dynamodb::DynamodbTable {
            name: self.table_name,
            client: self.client,
        };
        let encoding_key = jsonwebtoken::EncodingKey::from_secret(&self.jwt_secret);
        let public_key = crate::handlers::jwks::PublicKey(self.jwt_public);
        let state = ServerState {
            dynamodb,
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
            .route("/admin/wipe-table", get(handlers::admin::wipe_table))
            .route("/admin/scan-table", get(handlers::admin::scan_table))
            .route(
                "/applications",
                get(handlers::application::list_applications)
                    .post(handlers::application::create_application),
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
    client: Option<aws_sdk_dynamodb::Client>,
    table_name: Option<String>,
    jwt_secret: Option<Vec<u8>>,
    jwt_public: Option<Vec<u8>>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            addr: None,
            client: None,
            table_name: None,
            jwt_secret: None,
            jwt_public: None,
        }
    }

    pub fn addr(mut self, addr: SocketAddr) -> Self {
        self.addr = Some(addr);
        self
    }

    pub fn client(mut self, client: aws_sdk_dynamodb::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub fn table_name(mut self, table_name: String) -> Self {
        self.table_name = Some(table_name);
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
        let client = self.client.ok_or(error::Error::ServerBuilder)?;
        let table_name = self.table_name.ok_or(error::Error::ServerBuilder)?;
        let jwt_secret = self.jwt_secret.ok_or(error::Error::ServerBuilder)?;
        let jwt_public = self.jwt_public.ok_or(error::Error::ServerBuilder)?;

        Ok(Server {
            addr,
            client,
            table_name,
            jwt_secret,
            jwt_public,
        })
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}
