use crate::core_mod::core::create_config;
use crate::core_mod::widgets::SelectedItem;
use crate::core_mod::widgets::TableWidget;
use crate::screens::dashboard::Data;
use crate::screens::error::error_widget::ErrorType;
use crate::screens::error::error_widget::ErrorWidget;
use crate::screens::home::Home;
use crate::screens::popup::InputBox;
use crate::screens::popup::InputMode;
use crate::screens::popup::FLAG;
use crate::screens::protocol_popup::protocol_popup::ConnectionPopup;
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
    handle_char_key(home, key, input_box);
}

pub fn handle_q_key(home: &mut Home, input_box: &mut InputBox) {
    if home.show_api_popup {
        handle_char_key(home, 'q', input_box);
    } else if home.show_popup {
        home.popup_tx
            .send((home.selected_button as u16, Some(false)))
            .unwrap();
        home.show_popup = false;
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
        handle_char_key(home, c, input_box);
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
        home.current_screen = ScreenState::Connection;
        // Reset any other states that might interfere
        home.show_popup = false;
        home.show_api_popup = false;
        home.render_url_popup = false;
        input_box.input_mode = InputMode::Normal;
        unsafe { FLAG = false };
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
    }
}

pub fn handle_left_key(
    home: &mut Home,
    input_box: &mut InputBox,
    connection: &mut ConnectionPopup,
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
    }
}

pub fn handle_enter_key(
    home: &mut Home,
    input_box: &mut InputBox,
    error: &mut ErrorWidget,
    table: &mut TableWidget,
    connection: &mut ConnectionPopup,
) {
    match (home.show_popup, table.active, connection.visible) {
        (true, _, _) => {
            home.popup_tx
                .send((home.selected_button as u16, Some(true)))
                .unwrap();
            home.show_popup = false;
        }
        (_, true, _) => {
            let selected = table.enter();
            let data_item: Option<&Vec<Data>> = if let Some(SelectedItem::Device(device)) = selected
            {
                device.files.as_ref()
            } else {
                None
            };

            if let Some(data) = data_item {
                let cloned_items: Vec<(String, ratatui::prelude::Line, String, String)> = data
                    .iter()
                    .map(|d| {
                        (
                            d.name.clone(),
                            d.status.clone(),
                            d.destination.clone(),
                            d.time.clone(),
                        )
                    })
                    .collect();

                cloned_items
                    .into_iter()
                    .for_each(|(name, status, destination, time)| {
                        table.add_item(name, status, destination, time);
                    });
            }

            home.current_screen = ScreenState::Transfer;
        }
        (_, _, true) => {
            connection.input_popup = true;
            home.current_screen = ScreenState::TCP;
            connection.return_selected();
        }
        _ => match_input_state(home, input_box, error),
    }
}
fn match_input_state(home: &mut Home, input_box: &mut InputBox, error: &mut ErrorWidget) {
    match input_box.input_mode == InputMode::Editing {
        true => {
            let api = input_box.submit_message();
            match api {
                Ok(api) => {
                    create_config(&api).unwrap();
                    home.show_api_popup = false;
                }
                Err(err) => {
                    home.show_api_popup = false;
                    error.set_val(err.to_string(), &mut ErrorType::Warning, "Ok".to_string());
                    home.error = true;
                }
            }
        }
        false => {
            let output = input_box.submit_message();
            match output {
                Ok(key) => {
                    create_config(&key).unwrap();
                    home.show_api_popup = false;
                }
                Err(err) => {
                    home.show_api_popup = false;
                    error.set_val(err.to_string(), &mut ErrorType::Warning, "Ok".to_string());
                    home.error = true;
                }
            }
        }
    }
}
pub fn handle_char_key(_: &mut Home, c: char, input_box: &mut InputBox) {
    if input_box.input_mode == InputMode::Editing {
        input_box.enter_char(c);
    } else if c == 'e' {
        input_box.input_mode = InputMode::Editing;
        unsafe { FLAG = true };
    }
}

pub fn handle_up_key(_: &mut Home, table: &mut TableWidget) {
    if !table.help {
        table.previous();
    }
}

pub fn handle_down_arrow(_: &mut Home, table: &mut TableWidget) {
    if !table.help {
        table.next();
    }
}

pub fn handle_backspace_key(_: &mut Home, input_box: &mut InputBox) {
    if input_box.input_mode == InputMode::Editing {
        input_box.delete_char();
    }
}
