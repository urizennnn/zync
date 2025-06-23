use once_cell::sync::Lazy;
use std::path::PathBuf;

pub static STORAGE_PATH: Lazy<PathBuf> = Lazy::new(|| get_user_home_dir().join("zync-storage"));

pub fn get_user_home_dir() -> PathBuf {
    match dirs::home_dir() {
        Some(path) => path,
        None => {
            log::warn!("Could not determine home directory. Using root (/) as fallback.");
            PathBuf::from("/")
        }
    }
}
