use tcp_client::utils::get_ip::get_local_ip;

use crate::core_mod::core::create_config;
use crate::core_mod::widgets::SelectedItem;
use crate::core_mod::widgets::TableWidget;
use crate::screens::debug::DebugScreen;
use crate::screens::error::error_widget::ErrorType;
use crate::screens::error::error_widget::ErrorWidget;
use crate::screens::home::Home;
use crate::screens::host_type::HostTypePopup;
use crate::screens::popup::InputBox;
use crate::screens::popup::InputMode;
use crate::screens::popup::FLAG;
use crate::screens::protocol_popup::ConnectionPopup;
use crate::screens::protocol_popup::ConnectionType;
use crate::state::state::ScreenState;

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
            .unwrap();
        home.show_popup = false;
    } else if connection.input_popup {
        connection.input_popup = false;
        home.current_screen = ScreenState::Sessions;
    }

    // If we're in TcpServer, go back to Sessions
    if home.current_screen == ScreenState::TcpServer {
        home.current_screen = ScreenState::Sessions;
    }
    // If we're in Transfer, also go to Sessions
    else if home.current_screen == ScreenState::Transfer {
        home.current_screen = ScreenState::Sessions;
    }
    // Otherwise, quit
    else {
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

    // If we're in Connection, go back to Sessions
    if home.current_screen == ScreenState::Connection {
        connection.visible = false;
        home.current_screen = ScreenState::Sessions;
        return;
    }

    // If we're in Sessions OR Transfer, go to Connection
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
    if home.show_popup {
        home.popup_tx
            .send((home.selected_button as u16, Some(false)))
            .unwrap();
        home.show_popup = false;
    } else if input_box.input_mode == InputMode::Editing {
        input_box.input_mode = InputMode::Normal;
        unsafe { FLAG = false };
    } else if home.show_api_popup {
        home.show_api_popup = false;
    } else if home.render_url_popup {
        home.render_url_popup = false;
    } else if home.error {
        home.error = false;
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
        home.popup_tx
            .send((home.selected_button as u16, None))
            .unwrap();
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
        home.popup_tx
            .send((home.selected_button as u16, None))
            .unwrap();
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
    error: &mut ErrorWidget,
    table: &mut TableWidget,
    connection: &mut ConnectionPopup,
    host: &mut HostTypePopup,
) {
    if home.show_popup {
        let _ = home
            .popup_tx
            .send((home.selected_button as u16, Some(true)));
        home.show_popup = false;
        return;
    }
    if table.active {
        match table.enter() {
            Some(selected) => match selected {
                SelectedItem::Device(device) => {
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
                _ => (),
            },
            _ => (),
        }
        home.current_screen = ScreenState::Transfer;
        return;
    }
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
    if host.visible {
        let selected = host.return_selected();
        if selected == crate::screens::host_type::HostType::SENDER {
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
    if home.current_screen == ScreenState::TcpServer {
        if let Ok(user_input) = input_box.submit_message() {
            if let Ok(port) = user_input.parse::<u16>() {
                if !(1024..=0xFFFF).contains(&port) {
                    error.set_val(
                        "Port must be between 1024 and 65535".to_string(),
                        &mut ErrorType::Warning,
                        "Ok".to_string(),
                    );
                    home.error = true;
                    return;
                }
                let ui_tx = home.ui_update_tx.clone();
                tokio::spawn(async move {
                    match tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await {
                        Ok(listener) => {
                            let local_ip = get_local_ip().unwrap_or("0.0.0.0".to_string());
                            let msg =
                                format!("Connection Opened\nYour IP: {}\nPort: {}", local_ip, port);
                            ui_tx
                                .send(crate::events::ui_update::UIUpdate::ShowPopup(msg))
                                .await
                                .unwrap();
                            if let Ok((_, _)) = listener.accept().await {
                                ui_tx
                                    .send(crate::events::ui_update::UIUpdate::SwitchScreen(
                                        ScreenState::Sessions,
                                    ))
                                    .await
                                    .unwrap();
                            }
                        }
                        Err(e) => {
                            ui_tx
                                .send(crate::events::ui_update::UIUpdate::ShowPopup(format!(
                                    "Error opening port: {}",
                                    e
                                )))
                                .await
                                .unwrap();
                        }
                    }
                });
            } else {
                error.set_val(
                    "Invalid port number".to_string(),
                    &mut ErrorType::Warning,
                    "Ok".to_string(),
                );
                home.error = true;
            }
        }
        return;
    }
    if home.current_screen == ScreenState::TcpClient {
        if let Ok(user_input) = input_box.submit_message() {
            let target = user_input;
            let ui_tx = home.ui_update_tx.clone();
            tokio::spawn(async move {
                match tokio::net::TcpStream::connect(format!("{}:{}", target, 8080)).await {
                    Ok(mut _stream) => {
                        ui_tx
                            .send(crate::events::ui_update::UIUpdate::ShowPopup(
                                "Connection Opened".to_string(),
                            ))
                            .await
                            .unwrap();
                        ui_tx
                            .send(crate::events::ui_update::UIUpdate::SwitchScreen(
                                ScreenState::Sessions,
                            ))
                            .await
                            .unwrap();
                    }
                    Err(e) => {
                        ui_tx
                            .send(crate::events::ui_update::UIUpdate::ShowPopup(format!(
                                "Error connecting: {}",
                                e
                            )))
                            .await
                            .unwrap();
                    }
                }
            });
        }
        return;
    }
    match input_box.submit_message() {
        Ok(api) => {
            if let Err(e) = create_config(&api) {
                error.set_val(e.to_string(), &mut ErrorType::Warning, "Ok".to_string());
                home.error = true;
            }
            connection.visible = false;
            host.visible = false;
            home.show_api_popup = false;
            home.show_popup = false;
            home.current_screen = ScreenState::Sessions;
        }
        Err(err) => {
            error.set_val(err.to_string(), &mut ErrorType::Warning, "Ok".to_string());
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

pub fn handle_backspace_key(input_box: &mut InputBox) {
    if input_box.input_mode == InputMode::Editing {
        input_box.delete_char();
    }
}
