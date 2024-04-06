use lockpad::config::Config;

#[derive(clap::Args, Debug)]
pub(crate) struct ServerCommand {
    #[clap(subcommand)]
    pub command: ServerCommands,

    #[arg(default_value = "0.0.0.0:5000", long, short)]
    pub addr: std::net::SocketAddr,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum ServerCommands {
    /// start the http server
    Http,
}

impl ServerCommand {
    pub(crate) async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config = Config::load()?;

        let pg_pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.postgres_url)
            .await?;

        let server = lockpad_http::Server::builder()
            .addr(self.addr)
            .pg_pool(pg_pool)
            .jwt_secret(config.secret_key.as_bytes().to_owned())
            .jwt_public(config.public_key.as_bytes().to_owned())
            .disable_signup(config.disable_signup)
            .build()?;

        match self.command {
            ServerCommands::Http => server.run().await?,
        }

        Ok(())
    }
}
