use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

pub mod error;
pub mod handlers;

use error::Result;
use handlers::{authorize, hello_world, login_screen};

pub struct Server {
    addr: SocketAddr,
}

impl Server {
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }

    pub fn with_addr(addr: [u8; 4], port: u16) -> Self {
        Self {
            addr: SocketAddr::new(addr.into(), port),
        }
    }

    pub async fn run(&self) -> Result<()> {
        let cors = tower_http::cors::CorsLayer::permissive();

        let app = Router::new()
            .route("/", get(hello_world))
            .route("/login", get(login_screen))
            .route("/authorize", post(authorize))
            .layer(cors);

        tracing::info!("Listening on {0}", self.addr);
        axum::Server::bind(&self.addr)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::with_addr([0, 0, 0, 0], 3000)
    }
}
