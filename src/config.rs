use once_cell::sync::OnceCell;
use std::path::PathBuf;

#[derive(Debug)]
pub struct AppConfig {
    pub pkglist_path: PathBuf,
}

pub static CONFIG: OnceCell<AppConfig> = OnceCell::new();
