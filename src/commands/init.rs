use std::{
    fs::File,
    io::{BufWriter, Write},
    process::Command,
};

use crate::{error, step, substep_last, success};

pub fn run() {
    step!("Initializing pkglist");

    let output = Command::new("pacman")
        .args(&["-Qe"])
        .output()
        .expect("Failed to execute pacman");

    if !output.status.success() {
        error!("pacman -Qe failed with: {:?}", output.status);
        std::process::exit(1);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let packages: Vec<&str> = stdout
        .lines()
        .map(|line| line.split_whitespace().next().unwrap())
        .collect();

    let file = File::create("pkglist").unwrap_or_else(|_| {
        error!("Could not create pkglist");
        std::process::exit(1);
    });

    let mut writer = BufWriter::new(file);

    substep_last!("Writing {} packages", packages.len());

    writeln!(writer, "[system]").unwrap();
    for pkg in &packages {
        writeln!(writer, "pack {}", pkg).unwrap();
    }

    success!("Created 'pkglist'");
}
