#[macro_export]
macro_rules! step {
    ($($arg:tt)*) => {
        println!("⏵  {}", format!($($arg)*));
    };
}

#[macro_export]
macro_rules! substep {
    ($($arg:tt)*) => {
        println!("    ├─ {}", format!($($arg)*));
    };
}

#[macro_export]
macro_rules! substep_last {
    ($($arg:tt)*) => {
        println!("    └─ {}", format!($($arg)*));
    };
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {
        println!("✔  {}", format!($($arg)*));
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        eprintln!("⚠️  {}", format!($($arg)*));
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        eprintln!("✖  {}", format!($($arg)*));
    }};
}
