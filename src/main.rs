mod cli;
mod commands;
mod utils;

use clap::Parser;
use cli::{Command, PacksyCli};
use commands::init;

fn main() {
    let args = PacksyCli::parse();

    match args.command {
        Some(Command::Init) => {
            init::run();
        }
        None => {
            eprintln!("No command provided. Use --help.");
        }
    }
}
