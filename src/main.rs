mod cli;
mod commands;

use clap::Parser;
use cli::{Command, PacksyCli};

fn main() {
    let args = PacksyCli::parse();

    match args.command {
        Some(Command::List) => {
            commands::list::run();
        }
        None => {
            eprintln!("No command provided. Use --help.");
        }
    }
}
