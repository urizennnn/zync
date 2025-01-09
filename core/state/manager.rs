use lib_tcp::{app::listen, methods::list};
use ratatui::Frame;

use crate::{
    core_mod::widgets::TableWidget,
    screens::{
        connection_progress::ConnectionProgress, dashboard::table_ui, help::help_popup::HelpPopup,
        home::Home, host_type::HostTypePopup, popup::InputBox, protocol_popup::ConnectionPopup,
        session::draw_session_table_ui,
    },
};

use super::state::{ScreenState, StateSnapshot};

pub async fn manage_state(state: &mut StateSnapshot<'_>, f: &mut Frame<'_>) {
    let StateSnapshot {
        home,
        table,
        help,
        connection,
        host,
        progress,
        input_box,
    } = state;
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
                listen().await;
            }
        }
        _ => {}
    }
}
