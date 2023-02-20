use crate::commands::{Commands, ServerCommands};
use clap::Parser;
use lockpad::{create_table, entity::PrimaryId, models::user::User};
use serde_dynamo::to_item;

mod commands;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let args = commands::Args::parse();
    match args.command {
        Commands::Server(server) => {
            let cmd = server.command;
            let server = lockpad_http::Server::default();

            match cmd {
                ServerCommands::Http => server.run().await?,
            }
        }
    }

    Ok(())
}

#[allow(dead_code)]
async fn create_table_and_user() -> Result<(), Box<dyn std::error::Error>> {
    let client = scylla_dynamodb::connect_dynamodb("http://localhost:8100".to_string()).await;
    create_table(&client, "users").await?;

    let all_tables = client.list_tables().send().await?;
    println!("{:?}", all_tables);

    let user = User {
        id: PrimaryId {
            pk: "pk".to_string(),
            sk: "sk".to_string(),
        },
        name: "name".to_string(),
    };
    println!("{:?}", user);
    let item = to_item(user)?;
    client
        .put_item()
        .table_name("users")
        .set_item(Some(item))
        .send()
        .await?;

    let all_items = client.scan().table_name("users").send().await?;

    println!("{:?}", all_items);
    Ok(())
}
