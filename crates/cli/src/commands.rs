pub(crate) mod key;
pub(crate) mod server;
use key::KeyCommand;
use server::ServerCommand;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum Commands {
    /// commands for running the server
    Server(ServerCommand),
    /// commands for generating keypairs
    Key(KeyCommand),
}
