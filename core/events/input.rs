use crate::core_mod::widgets::TableWidget;
use crate::internal::session_store; // NEW: for updating session files and records
use crate::screens::debug::DebugScreen;
use crate::screens::home::Home;
use crate::screens::host_type::{HostType, HostTypePopup};
use crate::screens::popup::{InputBox, InputMode, FLAG};
use crate::screens::protocol_popup::{ConnectionPopup, ConnectionType};
use crate::state::state::{ConnectionState, ScreenState};
use std::sync::Arc;

// Removed references to "TcpLogs"
use tcp_client::app::connect_sync;
use tcp_client::utils::get_ip::get_local_ip;
use tcp_server::tcp::tcp::TCP;

/// Toggles the help display of the table widget if the API popup is not active, and processes the provided character input.
///
/// This function checks the `home` state and, if no API popup is shown, toggles the `help` flag within the table widget.
/// It then forwards the supplied character key to update the input box via `handle_char_key`.
///
/// # Examples
///
/// ```rust
/// // Dummy structures for demonstration purposes.
/// struct Home {
///     show_api_popup: bool,
/// }
///
/// struct TableWidget {
///     help: bool,
/// }
///
/// struct InputBox;
///
/// // Dummy implementation of handle_char_key for the example.
/// fn handle_char_key(_key: char, _input_box: &mut InputBox) {}
///
/// // The implementation of handle_help_key as defined in the module.
/// fn handle_help_key(home: &mut Home, table: &mut TableWidget, key: char, input_box: &mut InputBox) {
///     if !home.show_api_popup {
///         table.help = !table.help;
///     }
///     handle_char_key(key, input_box);
/// }
///
/// fn main() {
///     let mut home = Home { show_api_popup: false };
///     let mut table = TableWidget { help: false };
///     let mut input_box = InputBox;
///
///     // Toggling help when the API popup is inactive.
///     handle_help_key(&mut home, &mut table, 'h', &mut input_box);
///     assert!(table.help);
///
///     // With the API popup active, the help state should remain unchanged.
///     home.show_api_popup = true;
///     handle_help_key(&mut home, &mut table, 'h', &mut input_box);
///     assert!(table.help);
/// }
/// ```
pub fn handle_help_key(
    home: &mut Home,
    table: &mut TableWidget,
    key: char,
    input_box: &mut InputBox,
) {
    if !home.show_api_popup {
        table.help = !table.help;
    }
    handle_char_key(key, input_box);
}

/// Handles the 'q' key press to update the application's state.
///
/// Depending on the current UI state:
/// - If an API popup is active, delegates the 'q' character to the input box handler.
/// - If a generic popup is visible, sends a cancellation signal and dismisses the popup.
/// - If a connection input popup is active, closes it and switches the screen to Sessions.
/// - If the current screen is TcpServer or Transfer, changes the screen to Sessions.
/// - Otherwise, signals the application to terminate by setting `running` to false.
///
/// # Examples
///
/// ```rust
/// // Assume Home, InputBox, and ConnectionPopup implement Default and that ScreenState::Other
/// // represents a state other than TcpServer or Transfer.
/// let mut home = Home::default();
/// let mut input_box = InputBox::default();
/// let mut connection = ConnectionPopup::default();
///
/// // Configure state with no active popups and a non-special screen.
/// home.show_api_popup = false;
/// home.show_popup = false;
/// connection.input_popup = false;
/// home.current_screen = ScreenState::Other;
/// home.running = true;
///
/// handle_q_key(&mut home, &mut input_box, &mut connection);
///
/// // With the above state, pressing 'q' should signal the application to terminate.
/// assert!(!home.running);
/// ```
pub fn handle_q_key(home: &mut Home, input_box: &mut InputBox, connection: &mut ConnectionPopup) {
    if home.show_api_popup {
        handle_char_key('q', input_box);
    } else if home.show_popup {
        home.popup_tx
            .send((home.selected_button as u16, Some(false)))
            .ok();
        home.show_popup = false;
    } else if connection.input_popup {
        connection.input_popup = false;
        home.current_screen = ScreenState::Sessions;
    } else if home.current_screen == ScreenState::TcpServer {
        home.current_screen = ScreenState::Sessions;
    } else if home.current_screen == ScreenState::Transfer {
        home.current_screen = ScreenState::Sessions;
    } else {
        home.running = false;
    }
}

pub fn handle_n_key(
    home: &mut Home,
    c: char,
    input_box: &mut InputBox,
    connection: &mut ConnectionPopup,
) {
    if !home.show_api_popup && !home.show_popup {
        home.show_popup = true;
    }
    if home.show_api_popup || connection.input_popup {
        handle_char_key(c, input_box);
        return;
    }
    if home.current_screen == ScreenState::Connection {
        connection.visible = false;
        home.current_screen = ScreenState::Sessions;
        return;
    }
    if home.current_screen == ScreenState::Sessions || home.current_screen == ScreenState::Transfer
    {
        connection.visible = true;
        home.render_url_popup = false;
        input_box.input_mode = InputMode::Normal;
        unsafe { FLAG = false };
        home.current_screen = ScreenState::Connection;
    }
}

pub fn handle_esc_key(home: &mut Home, input_box: &mut InputBox) {
    match home.current_screen {
        ScreenState::TcpServer => {
            home.current_screen = ScreenState::Sessions;
        }
        ScreenState::TcpClient => {
            home.current_screen = ScreenState::Sessions;
        }
        ScreenState::Connection => {
            home.current_screen = ScreenState::TCP;
        }
        ScreenState::TCP => {
            home.current_screen = ScreenState::Sessions;
        }
        _ => {
            if home.show_popup {
                home.popup_tx
                    .send((home.selected_button as u16, Some(false)))
                    .ok();
                home.show_popup = false;
            } else if input_box.input_mode == InputMode::Editing {
                input_box.input_mode = InputMode::Normal;
                unsafe {
                    FLAG = false;
                }
            } else if home.show_api_popup {
                home.show_api_popup = false;
            } else if home.render_url_popup {
                home.render_url_popup = false;
            } else if home.error {
                home.error = false;
            }
        }
    }
}

pub fn handle_right_key(
    home: &mut Home,
    input_box: &mut InputBox,
    connection: &mut ConnectionPopup,
    host: &mut HostTypePopup,
) {
    if home.show_popup {
        home.selected_button = (home.selected_button + 1) % 2;
        home.popup_tx.send((home.selected_button as u16, None)).ok();
    } else if input_box.input_mode == InputMode::Editing {
        input_box.move_cursor_right();
    } else if connection.visible {
        connection.next();
    } else if host.visible {
        host.next();
    }
}

/// Processes the left arrow key input across various UI elements.
///
/// Depending on the current state, this function will:
/// - Cycle through buttons in an active popup by updating the selected button in `home` and sending an update via its channel.
/// - Move the cursor left if the input box is in editing mode.
/// - Navigate to the previous option in the connection or host type popups when visible.
///
/// # Examples
///
/// ```rust
/// use std::sync::mpsc;
///
/// // Dummy implementations for demonstration purposes.
/// struct Home {
///     show_popup: bool,
///     selected_button: usize,
///     popup_tx: mpsc::Sender<(u16, Option<()>)>,
/// }
///
/// impl Home {
///     fn new(show_popup: bool, selected_button: usize, popup_tx: mpsc::Sender<(u16, Option<()>)>) -> Self {
///         Self { show_popup, selected_button, popup_tx }
///     }
/// }
///
/// #[derive(PartialEq)]
/// enum InputMode {
///     Normal,
///     Editing,
/// }
///
/// struct InputBox {
///     input_mode: InputMode,
/// }
///
/// impl InputBox {
///     fn new(input_mode: InputMode) -> Self {
///         Self { input_mode }
///     }
///
///     fn move_cursor_left(&mut self) {
///         // Simulate moving the cursor left.
///     }
/// }
///
/// struct ConnectionPopup {
///     visible: bool,
/// }
///
/// impl ConnectionPopup {
///     fn new(visible: bool) -> Self {
///         Self { visible }
///     }
///
///     fn previous(&mut self) {
///         // Simulate navigating to the previous connection option.
///     }
/// }
///
/// struct HostTypePopup {
///     visible: bool,
/// }
///
/// impl HostTypePopup {
///     fn new(visible: bool) -> Self {
///         Self { visible }
///     }
///
///     fn previous(&mut self) {
///         // Simulate navigating to the previous host option.
///     }
/// }
///
/// // Assume `handle_left_key` is defined in the same scope.
/// fn handle_left_key(
///     home: &mut Home,
///     input_box: &mut InputBox,
///     connection: &mut ConnectionPopup,
///     host: &mut HostTypePopup,
/// ) {
///     if home.show_popup {
///         home.selected_button = (home.selected_button + 1) % 2;
///         home.popup_tx.send((home.selected_button as u16, None)).ok();
///     } else if input_box.input_mode == InputMode::Editing {
///         input_box.move_cursor_left();
///     } else if connection.visible {
///         connection.previous();
///     } else if host.visible {
///         host.previous();
///     }
/// }
///
/// fn main() {
///     let (tx, _rx) = mpsc::channel();
///     let mut home = Home::new(true, 0, tx);
///     let mut input_box = InputBox::new(InputMode::Normal);
///     let mut connection = ConnectionPopup::new(false);
///     let mut host = HostTypePopup::new(false);
///
///     // With a popup visible, the left arrow key cycles the button selection.
///     handle_left_key(&mut home, &mut input_box, &mut connection, &mut host);
///     assert_eq!(home.selected_button, 1);
/// }
/// ```
pub fn handle_left_key(
    home: &mut Home,
    input_box: &mut InputBox,
    connection: &mut ConnectionPopup,
    host: &mut HostTypePopup,
) {
    if home.show_popup {
        home.selected_button = (home.selected_button + 1) % 2;
        home.popup_tx.send((home.selected_button as u16, None)).ok();
    } else if input_box.input_mode == InputMode::Editing {
        input_box.move_cursor_left();
    } else if connection.visible {
        connection.previous();
    } else if host.visible {
        host.previous();
    }
}

/// Handles the Enter key press by dispatching actions based on the current UI state and input context.
///
/// This function processes the Enter key event by routing the action to one of several code paths:
/// - If a popup is active, it confirms the popup selection.
/// - If a device is selected in the table widget, it updates the table with file details and transitions to the transfer screen.
/// - If the connection or host type popups are visible, it adjusts the UI to begin a TCP connection in server or client mode.
/// - In TCP server mode, it validates the user-entered port, updates connection progress, spawns a thread to accept incoming connections,
///   and updates the session record and in-memory session table.
/// - In TCP client mode, it formats the user-provided address, initiates a connection in a spawned thread, and updates session records similarly.
/// - Otherwise, it handles API configuration input, updating the configuration and UI accordingly.
///
/// Note that this function spawns threads for blocking network operations and session updates, and it modifies several shared UI components.
///
/// # Examples
///
/// ```
/// use std::sync::{Arc, Mutex};
/// // Assuming default implementations for testing purposes.
/// let mut home = Home::default();
/// let mut input_box = InputBox::default();
/// let mut error = crate::screens::error::error_widget::ErrorWidget::default();
/// let mut table = TableWidget::default();
/// let connection_popup = Arc::new(Mutex::new(ConnectionPopup::default()));
/// let mut host = HostTypePopup::default();
/// let progress = Arc::new(Mutex::new(crate::screens::connection_progress::ConnectionProgress::default()));
///
/// // Example: Simulate API configuration input branch.
/// home.current_screen = ScreenState::Sessions;
/// input_box.input = "api_command".to_string();
///
/// handle_enter_key(
///     &mut home,
///     &mut input_box,
///     &mut error,
///     &mut table,
///     connection_popup.clone(),
///     &mut host,
///     progress.clone(),
/// );
///
/// // Verify that the screen transitioned back to Sessions.
/// assert_eq!(home.current_screen, ScreenState::Sessions);
/// ```pub fn handle_enter_key(
    home: &mut Home,
    input_box: &mut InputBox,
    error: &mut crate::screens::error::error_widget::ErrorWidget,
    table: &mut TableWidget,
    connection_arc: Arc<std::sync::Mutex<ConnectionPopup>>,
    host: &mut HostTypePopup,
    progress: Arc<std::sync::Mutex<crate::screens::connection_progress::ConnectionProgress>>,
) {
    let mut connection = connection_arc.lock().unwrap();
    if home.show_popup {
        if let Err(e) = home
            .popup_tx
            .send((home.selected_button as u16, Some(true)))
        {
            eprintln!("Failed to send popup confirmation: {}", e);
        }
        home.show_popup = false;
        return;
    }

    if table.active {
        if let Some(selected) = table.enter() {
            if let crate::core_mod::widgets::SelectedItem::Device(device) = selected {
                if let Some(files) = device.files.clone() {
                    for file in files {
                        table.add_item(
                            file.name.clone(),
                            file.status.clone(),
                            file.destination.clone(),
                            file.time.clone(),
                        );
                    }
                }
            }
        }
        home.current_screen = ScreenState::Transfer;
        return;
    }

    // If the main popup is open
    if connection.visible {
        let selected = connection.return_selected();
        if selected == Some(ConnectionType::TCP) {
            connection.input_popup = true;
            connection.visible = false;
            host.visible = true;
            home.current_screen = ScreenState::TCP;
        }
        return;
    }

    // If the HostType popup is open
    if host.visible {
        let selected = host.return_selected();
        if selected == HostType::SENDER {
            connection.logs = true;
            connection.visible = false;
            host.visible = false;
            home.current_screen = ScreenState::TcpServer;
        } else {
            connection.input_popup = true;
            connection.visible = false;
            host.visible = false;
            home.current_screen = ScreenState::TcpClient;
        }
        return;
    }

    // --- Server port logic ---
    if home.current_screen == ScreenState::TcpServer {
        if let Ok(user_input) = input_box.submit_message() {
            if let Ok(port) = user_input.parse::<u16>() {
                if !(1024..=65535).contains(&port) {
                    error.set_val(
                        "Port must be between 1024 and 65535".to_string(),
                        &mut crate::screens::error::error_widget::ErrorType::Warning,
                        "Ok".to_string(),
                    );
                    home.error = true;
                    return;
                }
                {
                    // Update progress state directly to Connecting
                    let mut prog = progress.lock().unwrap();
                    prog.state = ConnectionState::Connecting;
                }
                // Spawn a thread to accept connection and update progress and session store
                std::thread::spawn({
                    let progress_clone = progress.clone();
                    move || match TCP::accept_connection_sync(&format!("0.0.0.0:{}", port)) {
                        Ok((_socket, _addr)) => {
                            let mut prog = progress_clone.lock().unwrap();
                            prog.state = ConnectionState::Connected;
                            // After connection is accepted, update the session file.
                            let hostname = whoami::username();
                            let ip = get_local_ip().unwrap_or_else(|| "unknown".to_string());
                            let now = chrono::Utc::now().to_rfc3339();
                            let new_record = session_store::SessionRecord {
                                name: hostname.clone(),
                                ip: ip.clone(),
                                last_transfer: "N/A".to_string(),
                                last_connection: now.clone(),
                            };
                            session_store::update_session_record(new_record);
                        }
                        Err(e) => {
                            let mut prog = progress_clone.lock().unwrap();
                            prog.state =
                                ConnectionState::Failed(format!("Error opening port: {}", e));
                        }
                    }
                });
                // In the main thread, update the in-memory session table.
                let hostname = whoami::username();
                let ip = get_local_ip().unwrap_or_else(|| "unknown".to_string());
                let now = chrono::Utc::now().to_rfc3339();
                let new_device = crate::screens::session::Device {
                    name: hostname.clone(),
                    ip: ip.clone(),
                    last_transfer: crate::screens::session::Transfer {
                        status: "N/A".to_string(),
                        size: "N/A".to_string(),
                        name: "N/A".to_string(),
                    },
                    last_connection: crate::screens::session::Connection {
                        total: now.clone(),
                        format_date: now.clone(),
                    },
                    files: None,
                };
                let mut found = false;
                for item in table.items.iter_mut() {
                    if let crate::core_mod::widgets::Item::Device(ref mut d) = item {
                        if d.name == hostname {
                            d.ip = ip.clone();
                            d.last_connection.total = now.clone();
                            d.last_connection.format_date = now.clone();
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    table
                        .items
                        .push(crate::core_mod::widgets::Item::Device(new_device));
                }
                input_box.input.clear();
                input_box.reset_cursor();
                unsafe {
                    FLAG = false;
                }
            } else {
                error.set_val(
                    "Invalid port number".to_string(),
                    &mut crate::screens::error::error_widget::ErrorType::Warning,
                    "Ok".to_string(),
                );
                home.error = true;
            }
        }
        return;
    }

    // --- Client address logic ---
    if home.current_screen == ScreenState::TcpClient {
        if let Ok(user_input) = input_box.submit_message() {
            let address = if user_input.contains(':') {
                user_input.clone()
            } else {
                format!("{}:{}", user_input, 8080)
            };
            {
                let mut prog = progress.lock().unwrap();
                prog.state = ConnectionState::Connecting;
            }
            std::thread::spawn({
                let progress_clone = progress.clone();
                move || match connect_sync(&address) {
                    Ok(_stream) => {
                        let mut prog = progress_clone.lock().unwrap();
                        prog.state = ConnectionState::Connected;
                        // Update session record on success.
                        let hostname = whoami::username();
                        let ip = get_local_ip().unwrap_or_else(|| "unknown".to_string());
                        let now = chrono::Utc::now().to_rfc3339();
                        let new_record = session_store::SessionRecord {
                            name: hostname.clone(),
                            ip: ip.clone(),
                            last_transfer: "N/A".to_string(),
                            last_connection: now.clone(),
                        };
                        session_store::update_session_record(new_record);
                    }
                    Err(e) => {
                        let mut prog = progress_clone.lock().unwrap();
                        prog.state = ConnectionState::Failed(format!("Error connecting: {}", e));
                    }
                }
            });
            // Update in-memory session table as before.
            let hostname = whoami::username();
            let ip = get_local_ip().unwrap_or_else(|| "unknown".to_string());
            let now = chrono::Utc::now().to_rfc3339();
            let new_device = crate::screens::session::Device {
                name: hostname.clone(),
                ip: ip.clone(),
                last_transfer: crate::screens::session::Transfer {
                    status: "N/A".to_string(),
                    size: "N/A".to_string(),
                    name: "N/A".to_string(),
                },
                last_connection: crate::screens::session::Connection {
                    total: now.clone(),
                    format_date: now.clone(),
                },
                files: None,
            };
            let mut found = false;
            for item in table.items.iter_mut() {
                if let crate::core_mod::widgets::Item::Device(ref mut d) = item {
                    if d.name == hostname {
                        d.ip = ip.clone();
                        d.last_connection.total = now.clone();
                        d.last_connection.format_date = now.clone();
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                table
                    .items
                    .push(crate::core_mod::widgets::Item::Device(new_device));
            }
        }
        return;
    }

    // Otherwise handle normal input (i.e. the API scenario)
    match input_box.submit_message() {
        Ok(api) => {
            if let Err(e) = crate::core_mod::core::create_config(&api) {
                error.set_val(
                    e.to_string(),
                    &mut crate::screens::error::error_widget::ErrorType::Warning,
                    "Ok".to_string(),
                );
                home.error = true;
            }
            connection.visible = false;
            host.visible = false;
            home.show_api_popup = false;
            home.show_popup = false;
            home.current_screen = ScreenState::Sessions;
        }
        Err(err) => {
            error.set_val(
                err.to_string(),
                &mut crate::screens::error::error_widget::ErrorType::Warning,
                "Ok".to_string(),
            );
            home.error = true;
            connection.visible = false;
            host.visible = false;
            home.current_screen = ScreenState::Sessions;
        }
    }
}

pub fn handle_d_key(home: &mut Home, debug: &mut DebugScreen) {
    if home.current_screen == ScreenState::Debug {
        home.current_screen = ScreenState::Sessions;
        debug.push_line("Leaving Debug mode");
    } else {
        home.current_screen = ScreenState::Debug;
        debug.push_line("Entering Debug mode");
    }
}

pub fn handle_char_key(c: char, input_box: &mut InputBox) {
    if input_box.input_mode == InputMode::Editing {
        input_box.enter_char(c);
    } else if c == 'e' {
        input_box.input_mode = InputMode::Editing;
        unsafe { FLAG = true };
    }
}

pub fn handle_up_key(table: &mut TableWidget) {
    if !table.help {
        table.previous();
    }
}

pub fn handle_down_arrow(table: &mut TableWidget) {
    if !table.help {
        table.next();
    }
}

/// Removes the last character from the input box if it is in editing mode.
///
/// This function checks whether the input box is currently set to the `Editing` mode.
/// If so, it invokes the deletion of the last character. No action is performed if the
/// input box is not in editing mode.
///
/// # Examples
///
/// ```
/// // Assume `InputBox` and `InputMode` are defined and available.
///
/// // Create an input box instance and set it to editing mode.
/// let mut input_box = InputBox::new();
/// input_box.set_mode(InputMode::Editing);
/// input_box.set_text("Hello");
///
/// // Process a backspace key press, removing the last character.
/// handle_backspace_key(&mut input_box);
/// assert_eq!(input_box.get_text(), "Hell");
/// ```
pub fn handle_backspace_key(input_box: &mut InputBox) {
    if input_box.input_mode == InputMode::Editing {
        input_box.delete_char();
    }
}

/// Opens a file explorer for file selection when the current screen is set to Sessions.
///
/// This function checks if the application's active screen is Sessions and, if so,
/// launches the file explorer prompt for selecting a file.
///
/// # Examples
///
/// ```
/// // Assume Home and ScreenState are properly defined and initialized.
/// let mut home = Home::default();
/// home.current_screen = ScreenState::Sessions;
///
/// // This call will trigger the file explorer prompt.
/// handle_o_key(&mut home);
/// ```
pub fn handle_o_key(home: &mut Home) {
    if home.current_screen == ScreenState::Sessions {
        crate::internal::open_file::open_explorer_and_file_select();
    }
}
