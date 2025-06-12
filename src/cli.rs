use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "packsy", version, about = "Declarative package manager MVP")]
pub struct PacksyCli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    List,
    Init,
}
