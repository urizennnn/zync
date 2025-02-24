use crate::screens::debug::DebugScreen;
use crate::screens::host_type::HostType;
use crate::state::state::StateSnapshot;
use rfd::FileDialog;
use tcp_client::methods::upload::upload;

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
        if let Some(ref stream_arc) = state.stream {
            let mut stream = stream_arc.lock().unwrap();
            let mut buffer = vec![0u8; 209715200];

            // CHANGED: use the global runtime instead of creating a new one
            let result =
                crate::init::GLOBAL_RUNTIME.block_on(upload(&mut stream, &file_path, &mut buffer));

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
