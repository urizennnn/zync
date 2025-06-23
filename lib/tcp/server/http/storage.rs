use once_cell::sync::Lazy;
use std::path::PathBuf;

pub static STORAGE_PATH: Lazy<PathBuf> = Lazy::new(|| get_user_home_dir().join("zync-storage"));

/// Returns the current user's home directory path, or the root directory (`/`) if the home directory cannot be determined.
///
/// Logs a warning if the home directory is unavailable.
///
/// # Returns
/// The path to the user's home directory, or `/` as a fallback.
pub fn get_user_home_dir() -> PathBuf {
    match dirs::home_dir() {
        Some(path) => path,
        None => {
            log::warn!("Could not determine home directory. Using root (/) as fallback.");
            PathBuf::from("/")
        }
    }
}
