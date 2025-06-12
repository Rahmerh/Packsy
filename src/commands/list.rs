use std::process::Command;

pub fn run() {
    let output = Command::new("pacman")
        .args(&["-Qe"])
        .output()
        .expect("Failed to execute pacman");

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        for line in result.lines() {
            println!("{}", line);
        }
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}
