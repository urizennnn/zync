use crate::{
    core::core_lib::create_config,
    error::error_widget::ErrorWidget,
    home::homepage::Home,
    popup::{InputBox, InputMode},
    widget::TableWidget,
};

use crate::popup::FLAG;

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

pub fn handle_q_key(home: &mut Home, input_box: &mut InputBox, table: &mut TableWidget) {
    match (
        home.show_api_popup,
        home.show_popup,
        home.render_url_popup,
        table.help,
        table.connection,
    ) {
        (true, _, _, _, _) => handle_char_key(home, 'q', input_box),
        (_, true, _, _, _) => {
            home.popup_tx
                .send((home.selected_button as u16, Some(false)))
                .unwrap();
            home.show_popup = false;
        }
        (_, _, true, _, _) => {}
        (_, _, _, true, _) => {}
        (_, _, _, _, true) => {}
        _ => home.running = false,
    }
}

pub fn handle_n_key(home: &mut Home, c: char, input_box: &mut InputBox, table: &mut TableWidget) {
    if home.show_api_popup {
        handle_char_key(home, c, input_box);
    } else if !home.show_popup {
        home.show_popup = true;
    } else {
        table.connection = !table.connection;
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

pub fn handle_right_key(home: &mut Home, input_box: &mut InputBox) {
    if home.show_popup {
        home.selected_button = (home.selected_button + 1) % 2;
        home.popup_tx
            .send((home.selected_button as u16, None))
            .unwrap();
    } else if input_box.input_mode == InputMode::Editing {
        input_box.move_cursor_right();
    }
}

pub fn handle_left_key(home: &mut Home, input_box: &mut InputBox) {
    if home.show_popup {
        home.selected_button = (home.selected_button + 1) % 2;
        home.popup_tx
            .send((home.selected_button as u16, None))
            .unwrap();
    } else if input_box.input_mode == InputMode::Editing {
        input_box.move_cursor_left();
    }
}

pub fn handle_enter_key(home: &mut Home, input_box: &mut InputBox, error: &mut ErrorWidget) {
    if home.show_popup {
        home.popup_tx
            .send((home.selected_button as u16, Some(true)))
            .unwrap();
        home.show_popup = false;
    } else {
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
                        error.set_val(
                            err.to_string(),
                            &mut crate::error::error_widget::ErrorType::Warning,
                            "Ok".to_string(),
                        );
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
                        error.set_val(
                            err.to_string(),
                            &mut crate::error::error_widget::ErrorType::Warning,
                            "Ok".to_string(),
                        );
                        home.error = true;
                    }
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
    if !table.help && !table.connection {
        table.previous();
    }
}

pub fn handle_down_arrow(_: &mut Home, table: &mut TableWidget) {
    if !table.help && !table.connection {
        table.next();
    }
}

pub fn handle_backspace_key(_: &mut Home, input_box: &mut InputBox) {
    if input_box.input_mode == InputMode::Editing {
        input_box.delete_char();
    }
}
