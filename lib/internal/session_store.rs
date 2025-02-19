use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, create_dir_all},
    io::{Read, Write},
    path::PathBuf,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionRecord {
    pub name: String,
    pub ip: String,
    pub last_transfer: String,
    pub last_connection: String,
}

/// Returns the file path for today's session record.
/// On Unix, this will typically be ~/.local/share/zync/sessions/session_YYYY-MM-DD.json;
/// on Windows, it will be in the corresponding local app data directory.
fn get_session_file_path() -> PathBuf {
    // Use the local data directory (Unix: ~/.local/share, Windows: %LOCALAPPDATA%)
    let mut dir = dirs::data_local_dir().unwrap_or_else(|| std::env::current_dir().unwrap());
    // Create a subdirectory for your application
    dir.push("zync");
    // And a folder for session records
    dir.push("sessions");
    let date = Utc::now().format("%Y-%m-%d").to_string();
    dir.push(format!("session_{}.json", date));
    dir
}

pub fn load_sessions() -> Vec<SessionRecord> {
    let path = get_session_file_path();
    if path.exists() {
        let mut file = fs::File::open(&path).expect("Failed to open session file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read session file");
        serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

pub fn save_sessions(sessions: &[SessionRecord]) {
    let path = get_session_file_path();
    if let Some(parent) = path.parent() {
        create_dir_all(parent).expect("Failed to create sessions directory");
    }
    let contents =
        serde_json::to_string_pretty(sessions).expect("Failed to serialize session records");
    fs::write(path, contents).expect("Failed to write session file");
}

pub fn update_session_record(new_record: SessionRecord) {
    let mut sessions = load_sessions();
    let mut found = false;
    for record in sessions.iter_mut() {
        if record.name == new_record.name {
            record.ip = new_record.ip.clone();
            record.last_connection = new_record.last_connection.clone();
            found = true;
            break;
        }
    }
    if !found {
        sessions.push(new_record);
    }
    save_sessions(&sessions);
}
