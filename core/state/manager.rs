use super::state::{ScreenState, StateSnapshot};
use crate::screens::{dashboard::table_ui, session::draw_session_table_ui};
use ratatui::Frame;

pub async fn manage_state<'a>(mut state_snapshot: StateSnapshot<'a>, frame: &mut Frame<'_>) {
    // Update progress
    {
        let mut progress_guard = state_snapshot.progress.lock().await;
        progress_guard.update().await;
    }

    // Get current screen state
    let current_screen = state_snapshot.home.current_screen.clone();

    match current_screen {
        ScreenState::Sessions => {
            draw_session_table_ui(frame, state_snapshot.table, state_snapshot.home);
            if state_snapshot.table.help {
                state_snapshot.help.draw_dashboard_help(frame);
            }
        }
        ScreenState::Transfer => {
            table_ui(frame, state_snapshot.table);
            if state_snapshot.table.help {
                state_snapshot.help.draw_dashboard_help(frame);
            }
        }
        ScreenState::Connection => {
            state_snapshot.table.active = false;
            if state_snapshot.connection.visible {
                state_snapshot.connection.render(frame);
            }
        }
        ScreenState::TCP => {
            state_snapshot.connection.visible = false;
            state_snapshot.table.active = false;
            if state_snapshot.host.visible {
                state_snapshot.host.render(frame);
            }
        }
        ScreenState::TcpServer => {
            let progress_guard = state_snapshot.progress.lock().await;
            progress_guard.draw(frame);
        }
        ScreenState::TcpClient => {
            if state_snapshot.connection.input_popup {
                state_snapshot.connection.draw_input(
                    frame,
                    state_snapshot.connection.returned_val.clone(),
                    state_snapshot.input_box,
                );
            }
        }
        _ => {}
    }
}
