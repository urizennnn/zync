use ratatui::Frame;

use crate::screens::{dashboard::table_ui, session::draw_session_table_ui};

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

    let progress_ref = progress.clone();
    tokio::spawn(async move {
        let mut guard = progress_ref.lock().await;
        guard.update().await;
    });
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
        }
        ScreenState::TcpServer => {
            if connection.logs {
                let progress_ref = progress.clone();
                tokio::spawn(async move {
                    let mut guard = progress_ref.lock().await;
                    guard.update().await;
                });
            }
        }
        ScreenState::TcpClient => {
            connection.input_popup = true;
            if connection.input_popup {
                connection.draw_input(f, connection.returned_val, input_box)
            }
        }
        _ => {}
    }
}
