use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "packsy",
    version,
    about = "Sync your Arch packages with a list—install what’s missing, clean what’s not.",
    long_about = "Packsy is a declarative package sync tool for Arch-based systems.\n\
It installs packages from a list and optionally removes those not listed—keeping your\n\
system clean, minimal, and reproducible. Ideal for maintaining consistent setups across machines."
)]
struct Args {
    #[arg(help = "Path to your package list file")]
    file: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let path = Path::new(&args.file);

    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}
