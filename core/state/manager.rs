use lib_tcp::app::listen;
use ratatui::Frame;

use crate::{
    core_mod::widgets::TableWidget,
    screens::{
        connection_progress::ConnectionProgress, dashboard::table_ui, help::help_popup::HelpPopup,
        home::Home, host_type::HostTypePopup, popup::InputBox, protocol_popup::ConnectionPopup,
        session::draw_session_table_ui,
    },
};

use super::state::ScreenState;

pub async fn manage_state(
    home: &mut Home,
    table: &mut TableWidget,
    f: &mut Frame<'_>,
    help: &mut HelpPopup,
    connection: &mut ConnectionPopup,
    input: &mut InputBox,
    host: &mut HostTypePopup,
    progress: &mut ConnectionProgress,
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
        }
        ScreenState::TCP => {
            connection.visible = false;
            table.active = false;
            if host.visible {
                host.render(f);
            }
            // if connection.input_popup {
            //     connection.draw_input(f, ConnectionType::TCP, input)
            // }
        }
        ScreenState::TcpLogs => {
            if connection.logs {
                progress.draw(f);
                tokio::spawn(listen()).await;
            }
        }
        _ => {}
    }
}
