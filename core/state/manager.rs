use super::state::{ScreenState, StateSnapshot};
use crate::{
    screens::{dashboard::table_ui, home::Home, session::draw_session_table_ui},
    utils::poll::poll_future,
};
use futures::lock::Mutex;
use ratatui::DefaultTerminal;
use std::sync::Arc;

pub async fn manage_state(
    home: &mut Home,
    state_snapshot: Arc<Mutex<StateSnapshot<'_>>>,
    term: Arc<Mutex<DefaultTerminal>>,
) -> Result<(), std::io::Error> {
    let progress = Arc::clone(state_snapshot.lock().await.progress);

    tokio::spawn(async move {
        let mut progress = progress.lock().await;
        progress.update().await;
    });

    let mut state_guard = state_snapshot.lock().await;

    term.lock().await.draw(|frame| match home.current_screen {
        ScreenState::Sessions => {
            draw_session_table_ui(frame, state_guard.table);
            if state_guard.table.help {
                state_guard.help.draw_dashboard_help(frame);
            }
        }
        ScreenState::Transfer => {
            table_ui(frame, state_guard.table);
            if state_guard.table.help {
                state_guard.help.draw_dashboard_help(frame);
            }
        }
        ScreenState::Connection => {
            state_guard.table.active = false;
            if state_guard.connection.visible {
                state_guard.connection.render(frame);
            }
        }
        ScreenState::TCP => {
            state_guard.connection.visible = false;
            state_guard.table.active = false;
            if state_guard.host.visible {
                state_guard.host.render(frame);
            }
        }
        ScreenState::TcpServer => {
            let progress = poll_future(Box::pin(state_guard.progress.lock()));
            progress.draw(frame);
        }
        // ScreenState::TcpClient => {
        //     if state_guard.connection.input_popup {
        //         state_guard.connection.draw_input(
        //             frame,
        //             state_guard.connection.returned_val,
        //             state_guard.input_box,
        //         );
        //     }
        // }
        _ => {}
    })?;

    Ok(())
}
