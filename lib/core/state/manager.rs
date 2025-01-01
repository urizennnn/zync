use ratatui::Frame;

use crate::{
    dashboard::dashboard_view::table_ui, help::help_popup::HelpPopup, home::homepage::Home,
    protocol::protocol_popup::ConnectionPopup, sessions::draw_session_table_ui, state::ScreenState,
    widget::TableWidget,
};

pub fn manage_state(
    home: &mut Home,
    table: &mut TableWidget,
    f: &mut Frame,
    help: &mut HelpPopup,
    connection: &mut ConnectionPopup,
) {
    match home.current_screen {
        ScreenState::Sessions => {
            draw_session_table_ui(f, table, home);
            if table.help {
                help.draw_dashboard_help(f);
            }
        }
        ScreenState::Transfer => {
            table_ui(f, table);
            if table.help {
                help.draw_dashboard_help(f);
            }
        }
        ScreenState::Connection => {
            table.active = false;
            if connection.visible {
                connection.render(f);
            }
            if connection.input_popup {
                connection.draw_input(f)
            }
        }
        _ => {}
    }
}
