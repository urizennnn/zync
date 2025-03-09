use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File, create_dir_all},
    io::Read,
    path::PathBuf,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionRecord {
    pub name: String,
    pub ip: String,
    pub last_transfer: String,
    pub last_connection: String,
}

fn get_session_file_path() -> PathBuf {
    let mut dir = dirs::data_local_dir().unwrap_or_else(|| std::env::current_dir().unwrap());
    dir.push("zync");
    dir.push("sessions");
    let date = Utc::now().format("%Y-%m-%d").to_string();
    dir.push(format!("session_{}.json", date));
    dir
}

/// This helper gives us just the "zync/sessions" directory
fn get_session_dir_path() -> PathBuf {
    let mut dir = dirs::data_local_dir().unwrap_or_else(|| std::env::current_dir().unwrap());
    dir.push("zync");
    dir.push("sessions");
    dir
}

/// Loads *all* session records found in the "zync/sessions" directory
pub fn load_sessions() -> Vec<SessionRecord> {
    let session_dir = get_session_dir_path();
    let mut all_records = Vec::new();

    if session_dir.exists() {
        if let Ok(entries) = fs::read_dir(&session_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(fname) = path.file_name().and_then(|f| f.to_str()) {
                        // Only consider files named "session_... .json"
                        if fname.starts_with("session_") && fname.ends_with(".json") {
                            let mut contents = String::new();
                            if let Ok(mut file) = File::open(&path) {
                                if file.read_to_string(&mut contents).is_ok() {
                                    match serde_json::from_str::<Vec<SessionRecord>>(&contents) {
                                        Ok(records) => {
                                            all_records.extend(records);
                                        }
                                        Err(_) => {
                                            // skip corrupted or non-Vec JSON
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    all_records
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
    // We load *all* existing session records first:
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

    // By default, `save_sessions` still writes only to the “current day’s” session file.
    // If you’d like to store new records differently, update `save_sessions` accordingly.
    save_sessions(&sessions);
}
