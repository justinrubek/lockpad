use lockpad::config::Config;
use lockpad::create_table;
use tracing::info;

#[derive(clap::Args, Debug)]
pub(crate) struct ServerCommand {
    #[clap(subcommand)]
    pub command: ServerCommands,

    #[arg(default_value = "0.0.0.0:3000", long, short)]
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
        info!(?config, "loaded configuration");

        let dynamo_client = scylla_dynamodb::connect_dynamodb(config.dynamodb_endpoint).await;
        create_table_if_not_exists(&dynamo_client, &config.dynamodb_table).await?;

        let server = lockpad_http::Server::builder()
            .addr(self.addr)
            .client(dynamo_client.clone())
            .table_name(config.dynamodb_table)
            .jwt_secret(config.secret_key.as_bytes().to_owned())
            .jwt_public(config.public_key.as_bytes().to_owned())
            .build()?;

        match self.command {
            ServerCommands::Http => server.run().await?,
        }

        Ok(())
    }
}

async fn create_table_if_not_exists(
    client: &aws_sdk_dynamodb::Client,
    table_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    info!(?table_name, "checking if table exists");
    let all_tables = client.list_tables().send().await?;
    if !all_tables
        .table_names()
        .unwrap()
        .contains(&table_name.to_string())
    {
        info!(?table_name, "table does not exist, creating");
        create_table(client, table_name).await?;
    }

    Ok(())
}
