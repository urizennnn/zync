use crate::init::GLOBAL_RUNTIME;
use crate::screens::debug::DebugScreen;
use crate::screens::host_type::HostType;
use crate::state::state::StateSnapshot;
use log::error;
use rfd::FileDialog;
use std::fs;
use tcp_client::methods::upload::upload;
use tokio::io::AsyncWriteExt;

pub fn open_explorer_and_file_select(state: &StateSnapshot, debug_screen: &mut DebugScreen) {
    let host = state.host.lock().unwrap();
    if host.selected != HostType::SENDER {
        debug_screen.push_line("File sending is only available in client mode.");
        return;
    }
    drop(host);

    if let Some(path) = FileDialog::new().pick_file() {
        debug_screen.push_line(format!("Selected file: {}", path.display()));
        let file_path = path.to_string_lossy().into_owned();

        // Obtain the file name from the selected file
        let file_name = match path.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => {
                debug_screen.push_line("Invalid file name.".to_string());
                return;
            }
        };

        // Get file metadata (size)
        let metadata = fs::metadata(&path);
        let file_size = match metadata {
            Ok(meta) => meta.len(),
            Err(e) => {
                debug_screen.push_line(format!("Error reading file metadata: {}", e));
                return;
            }
        };

        // Build the command string using the new protocol format
        let command = format!("PUT {} {}\n", file_name, file_size);
        debug_screen.push_line(format!("Sending command: {}", command.trim()));

        if let Some(ref stream_arc) = state.stream {
            let mut stream = stream_arc.lock().unwrap();
            let mut buffer = vec![0u8; 209715200];

            // Send the command to the server
            GLOBAL_RUNTIME.block_on(async {
                match stream.write_all(command.as_bytes()).await {
                    Ok(_) => log::debug!("Sent command: {}", command.trim()),
                    Err(e) => error!("Error writing command to stream: {e}"),
                }
                stream.flush().await.ok();
            });

            // Call the upload function to send the file data
            let result = GLOBAL_RUNTIME.block_on(upload(&mut stream, &file_path, &mut buffer));

            match result {
                Ok(_) => debug_screen.push_line("File uploaded successfully.".to_string()),
                Err(e) => debug_screen.push_line(format!("Error uploading file: {}", e)),
            }
        } else {
            debug_screen.push_line("No active TCP connection available.".to_string());
        }
    } else {
        debug_screen.push_line("No file selected.".to_string());
    }
}
