use super::state::{ScreenState, StateSnapshot};
use crate::screens::{dashboard::table_ui, home::Home, session::draw_session_table_ui};
use futures::lock::Mutex;
use ratatui::DefaultTerminal;
use std::sync::Arc;

pub async fn manage_state(
    home: &mut Home,
    state_snapshot: Arc<Mutex<StateSnapshot<'_>>>,
    term: Arc<Mutex<DefaultTerminal>>,
) -> Result<(), std::io::Error> {
    let mut state_guard = state_snapshot.lock().await;
    tokio::spawn(async move {});

    if home.current_screen == ScreenState::TcpServer {
        let progress = Arc::clone(state_guard.progress);
        let term_clone = Arc::clone(&term);

        tokio::spawn(async move {
            let mut guard = progress.lock().await;
            term_clone
                .lock()
                .await
                .draw(|frame| {
                    guard.draw(frame);
                })
                .unwrap();
        });
    }

    term.lock().await.draw(|frame| {
        let state = &mut *state_guard;

        match home.current_screen {
            ScreenState::Sessions => {
                draw_session_table_ui(frame, state.table);
                if state.table.help {
                    state.help.draw_dashboard_help(frame);
                }
            }
            ScreenState::Transfer => {
                table_ui(frame, state.table);
                if state.table.help {
                    state.help.draw_dashboard_help(frame);
                }
            }
            ScreenState::Connection => {
                state.table.active = false;
                if state.connection.visible {
                    state.connection.render(frame);
                }
            }
            ScreenState::TCP => {
                state.connection.visible = false;
                state.table.active = false;
                if state.host.visible {
                    state.host.render(frame);
                }
            }
            _ => {}
        }
    })?;

    Ok(())
}
