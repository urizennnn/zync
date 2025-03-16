use std::path::PathBuf;

use once_cell::sync::Lazy;

pub static STORAGE_PATH: Lazy<PathBuf> = Lazy::new(|| get_user_home_dir().join("zync-storage"));

pub fn get_user_home_dir() -> PathBuf {
    match dirs::home_dir() {
        Some(path) => path,
        None => {
            println!("Warning: Could not determine home directory. Using root (/) as fallback.");
            PathBuf::from("/")
        }
    }
}
