use crate::core_mod::event::SyncTrait;
use crate::core_mod::widgets::TableWidget;
use crate::screens::debug::DebugScreen;
use crate::screens::home::Home;
use crate::screens::host_type::HostTypePopup;
use crate::screens::popup::InputBox;
use crate::screens::popup::InputMode;
use crate::screens::popup::FLAG;
use crate::screens::protocol_popup::ConnectionPopup;
use crate::state::state::ScreenState;
use std::sync::Arc;
use tcp_client::app::connect_sync;
use tcp_server::tcp::tcp::TCP;

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
    }
    if home.current_screen == ScreenState::TcpServer {
        home.current_screen = ScreenState::Sessions;
    }
    if home.current_screen == ScreenState::Transfer {
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
        ScreenState::TcpLogs => {
            home.current_screen = ScreenState::Connection;
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

pub fn handle_enter_key(
    home: &mut Home,
    input_box: &mut InputBox,
    error: &mut crate::screens::error::error_widget::ErrorWidget,
    table: &mut TableWidget,
    connection: Arc<std::sync::Mutex<ConnectionPopup>>,
    host: &mut HostTypePopup,
    progress: Arc<std::sync::Mutex<crate::screens::connection_progress::ConnectionProgress>>,
) {
    if home.show_popup {
        let _ = home
            .popup_tx
            .send((home.selected_button as u16, Some(true)));
        home.show_popup = false;
        return;
    }
    if table.active {
        if let Some(selected) = table.enter() {
            match selected {
                crate::core_mod::widgets::SelectedItem::Device(_device) => {
                    // handle device selection
                }
                _ => {}
            }
        }
        home.current_screen = ScreenState::Transfer;
        return;
    }

    // If user typed "server" scenario:
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
                    let sender = progress.lock().unwrap().get_event_sender().clone_sender();
                    sender
                        .send(crate::state::state::ConnectionState::Connecting)
                        .ok();
                }
                let ui_tx = home.ui_update_tx.clone();
                let progress_clone = progress.clone();
                std::thread::spawn(move || {
                    match TCP::accept_connection_sync(&format!("0.0.0.0:{}", port)) {
                        Ok((_socket, _addr)) => {
                            {
                                let sender = progress_clone
                                    .lock()
                                    .unwrap()
                                    .get_event_sender()
                                    .clone_sender();
                                sender
                                    .send(crate::state::state::ConnectionState::Connected)
                                    .ok();
                            }
                            ui_tx
                                .send(crate::events::ui_update::UIUpdate::SwitchScreen(
                                    ScreenState::TcpLogs,
                                ))
                                .ok();
                        }
                        Err(e) => {
                            let sender = progress_clone
                                .lock()
                                .unwrap()
                                .get_event_sender()
                                .clone_sender();
                            sender
                                .send(crate::state::state::ConnectionState::Failed(format!(
                                    "Error opening port: {}",
                                    e
                                )))
                                .ok();
                            ui_tx
                                .send(crate::events::ui_update::UIUpdate::SwitchScreen(
                                    ScreenState::TcpLogs,
                                ))
                                .ok();
                        }
                    }
                });
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

    // If user typed "client" scenario:
    if home.current_screen == ScreenState::TcpClient {
        if let Ok(user_input) = input_box.submit_message() {
            let address = if user_input.contains(':') {
                user_input.clone()
            } else {
                format!("{}:{}", user_input, 8080)
            };
            {
                let sender = progress.lock().unwrap().get_event_sender().clone_sender();
                sender
                    .send(crate::state::state::ConnectionState::Connecting)
                    .ok();
            }
            let ui_tx = home.ui_update_tx.clone();
            let progress_clone = progress.clone();
            std::thread::spawn(move || {
                // blocking connect using the tcp client library wrapper
                match connect_sync(&address) {
                    Ok(_stream) => {
                        let sender = progress_clone
                            .lock()
                            .unwrap()
                            .get_event_sender()
                            .clone_sender();
                        sender
                            .send(crate::state::state::ConnectionState::Connected)
                            .ok();
                        ui_tx
                            .send(crate::events::ui_update::UIUpdate::SwitchScreen(
                                ScreenState::TcpLogs,
                            ))
                            .ok();
                    }
                    Err(e) => {
                        let sender = progress_clone
                            .lock()
                            .unwrap()
                            .get_event_sender()
                            .clone_sender();
                        sender
                            .send(crate::state::state::ConnectionState::Failed(format!(
                                "Error connecting: {}",
                                e
                            )))
                            .ok();
                        ui_tx
                            .send(crate::events::ui_update::UIUpdate::SwitchScreen(
                                ScreenState::TcpLogs,
                            ))
                            .ok();
                    }
                }
            });
        }
        return;
    }

    // Otherwise, handle typed API key scenario:
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
            connection.lock().unwrap().visible = false;
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
            connection.lock().unwrap().visible = false;
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

pub fn handle_backspace_key(input_box: &mut InputBox) {
    if input_box.input_mode == InputMode::Editing {
        input_box.delete_char();
    }
}
