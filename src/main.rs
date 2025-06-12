mod cli;
mod commands;
mod config;

use crate::config::AppConfig;
use crate::config::CONFIG;
use clap::Parser;
use cli::{Command, PacksyCli};

fn main() {
    let pkglist_path = dirs_next::config_dir().unwrap().join("packsy/pkglist");

    CONFIG.set(AppConfig { pkglist_path }).unwrap();

    let args = PacksyCli::parse();

    match args.command {
        Some(Command::List) => {
            commands::list::run();
        }
        Some(Command::Init) => {
            commands::init::run();
        }
        None => {
            eprintln!("No command provided. Use --help.");
        }
    }
}
