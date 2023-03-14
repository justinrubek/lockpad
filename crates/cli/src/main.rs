use crate::commands::{Commands, ServerCommands};
use clap::Parser;
use commands::KeyCommands;
use lockpad::create_table;
use std::io::Write;
use tracing::info;

mod commands;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // TODO: load configuration from env/file
    let table_name = "lockpad-test-1";
    let dynamo_client =
        scylla_dynamodb::connect_dynamodb("http://localhost:8100".to_string()).await;
    create_table_if_not_exists(&dynamo_client, table_name).await?;

    let args = commands::Args::parse();
    match args.command {
        Commands::Server(server) => {
            let cmd = server.command;
            let server =
                lockpad_http::Server::new(server.addr, dynamo_client, table_name.to_string());

            match cmd {
                ServerCommands::Http => server.run().await?,
            }
        }

        Commands::Key(key) => {
            let cmd = key.command;

            match cmd {
                KeyCommands::Generate => {
                    let keypair = ed25519_compact::KeyPair::generate();

                    let mut file = std::fs::File::create("secret.der")?;
                    file.write_all(&keypair.sk.to_der())?;

                    let mut file = std::fs::File::create("public.der")?;
                    file.write_all(&keypair.pk.to_der())?;
                }
            }
        }
    }

    Ok(())
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
