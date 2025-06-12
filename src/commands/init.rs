use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    process::Command,
};

use crate::config::CONFIG;

pub fn run() {
    let output = Command::new("pacman")
        .args(&["-Qe"])
        .output()
        .expect("Failed to execute pacman");

    if !output.status.success() {
        eprintln!("pacman -Qe failed with: {:?}", output.status);
        std::process::exit(1);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let packages: Vec<&str> = stdout
        .lines()
        .map(|line| line.split_whitespace().next().unwrap())
        .collect();

    let config = CONFIG.get().expect("Appconfig not initialized");

    fs::create_dir_all(config.pkglist_path.parent().unwrap())
        .expect("Failed to create config directory");

    let file = File::create(&config.pkglist_path).expect("Failed to create file");
    let mut writer = BufWriter::new(file);

    writeln!(writer, "[system]").unwrap();
    for pkg in &packages {
        writeln!(writer, "pack {}", pkg).unwrap();
    }

    println!("Created packsy.lock with {} packages", packages.len());
}
