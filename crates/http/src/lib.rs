use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

pub mod error;
pub mod handlers;

use error::Result;
use handlers::{
    auth::{authorize, signup},
    pages::{login_screen, root, signup_screen},
    user::{get_user, list_users},
};

pub struct Server {
    addr: SocketAddr,
    client: aws_sdk_dynamodb::Client,
    table_name: String,
}

pub fn router() -> Router<scylla_dynamodb::DynamodbTable> {
    Router::new()
        .route("/", get(root))
        .route("/login", get(login_screen))
        .route("/signup-screen", get(signup_screen))
        .route("/authorize", post(authorize))
        .route("/signup", post(signup))
        .route("/users", get(list_users))
        .route("/users/:user_id", get(get_user))
        .route("/admin/wipe-table", get(handlers::admin::wipe_table))
        .route("/admin/scan-table", get(handlers::admin::scan_table))
        .route(
            "/applications",
            get(handlers::application::list_applications)
                .post(handlers::application::create_application),
        )
}

impl Server {
    pub fn new(addr: SocketAddr, client: aws_sdk_dynamodb::Client, table_name: String) -> Self {
        Self {
            addr,
            client,
            table_name,
        }
    }

    pub async fn run(self) -> Result<()> {
        let cors = tower_http::cors::CorsLayer::permissive();

        let dynamodb = scylla_dynamodb::DynamodbTable {
            name: self.table_name,
            client: self.client,
        };

        let app = router().with_state(dynamodb).layer(cors);

        tracing::info!("Listening on {0}", self.addr);
        axum::Server::bind(&self.addr)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}
