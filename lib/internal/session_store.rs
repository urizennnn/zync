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
/// Returns the file path for today's session record file.
///
/// This function constructs a path within the local data directory—using %LOCALAPPDATA% on Windows and ~/.local/share on Unix—by appending the "zync/sessions" subdirectory. The file name is based on the current UTC date in the format "session_YYYY-MM-DD.json". If the local data directory cannot be determined, it falls back to the current working directory.
///
/// # Examples
///
/// ```
/// let path = get_session_file_path();
/// println!("Session file path: {:?}", path);
/// ```fn get_session_file_path() -> PathBuf {
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

/// Loads session records from the session file.
///
/// This function constructs the session file path for the current date and attempts to open and
/// read its contents. If the file exists, it deserializes the JSON content into a vector of
/// `SessionRecord`. If the file does not exist or deserialization fails, an empty vector is returned.
///
/// # Examples
///
/// ```
/// // Assuming no valid session file is present, this will return an empty vector.
/// let sessions = load_sessions();
/// assert!(sessions.is_empty());
/// ```
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

/// Saves session records to a JSON file.
///
/// This function serializes the provided session records into a pretty-printed JSON format,
/// ensures that the parent directory exists, and writes the resulting data to the session file
/// determined by the current date. It will panic if directory creation, serialization, or file writing fails.
///
/// # Panics
///
/// Panics if the sessions directory cannot be created, the session records cannot be serialized,
/// or the file cannot be written.
///
/// # Examples
///
/// ```
/// use your_crate::session_store::{save_sessions, SessionRecord};
///
/// let sessions = vec![SessionRecord {
///     name: "session1".into(),
///     ip: "192.168.1.100".into(),
///     last_transfer: None,
///     last_connection: None,
/// }];
///
/// save_sessions(&sessions);
/// ```
pub fn save_sessions(sessions: &[SessionRecord]) {
    let path = get_session_file_path();
    if let Some(parent) = path.parent() {
        create_dir_all(parent).expect("Failed to create sessions directory");
    }
    let contents =
        serde_json::to_string_pretty(sessions).expect("Failed to serialize session records");
    fs::write(path, contents).expect("Failed to write session file");
}

/// Updates an existing session record or adds it if no record with the same name exists.
///
/// This function retrieves the current session records, searches for a record with a matching
/// `name`, and updates its `ip` and `last_connection` fields if found. If no matching record
/// exists, the new record is appended. Finally, the updated set of records is saved to storage.
///
/// # Examples
///
/// ```
/// use your_crate::session_store::{SessionRecord, update_session_record, load_sessions};
///
/// // Create a new session record for "user1".
/// let record = SessionRecord {
///     name: "user1".to_string(),
///     ip: "192.168.1.1".to_string(),
///     last_transfer: None,  // Adjust as necessary
///     last_connection: "2025-02-01T12:00:00Z".to_string(),
/// };
///
/// // Insert the new record.
/// update_session_record(record.clone());
///
/// // Update the record for "user1" with new connection details.
/// let updated_record = SessionRecord {
///     name: "user1".to_string(),
///     ip: "192.168.1.2".to_string(),
///     last_transfer: None,  // This field is not updated during record replacement.
///     last_connection: "2025-02-02T14:00:00Z".to_string(),
/// };
///
/// update_session_record(updated_record);
///
/// // Verify that the session record has been updated.
/// let sessions = load_sessions();
/// let session = sessions.iter().find(|r| r.name == "user1").expect("Session not found");
/// assert_eq!(session.ip, "192.168.1.2");
/// assert_eq!(session.last_connection, "2025-02-02T14:00:00Z");
/// ```
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
