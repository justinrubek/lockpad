#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum Commands {
    /// commands for running the server
    Server(Server),
    /// commands for generating keypairs
    Key(Key),
}

#[derive(clap::Args, Debug)]
pub(crate) struct Server {
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

#[derive(clap::Args, Debug)]
pub(crate) struct Key {
    #[clap(subcommand)]
    pub command: KeyCommands,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum KeyCommands {
    /// generate a new keypair
    Generate,
}
