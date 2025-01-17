use std::ops::DerefMut;
use std::sync::Arc;

use futures::lock::Mutex;
use ratatui::Frame;

use crate::screens::{dashboard::table_ui, session::draw_session_table_ui};

use super::state::{ScreenState, StateSnapshot};

// FIX: make frame be wrapped in arc<mutex<T>>
pub async fn manage_state(state: &mut StateSnapshot<'static>, f: &mut Frame<'static>) {
    let StateSnapshot {
        home,
        table,
        help,
        connection,
        host,
        progress,
        input_box,
    } = state;

    // Wrap the frame in a mutex for shared access
    let f_mutex = Arc::new(Mutex::new(f));

    // Spawn an async task to update progress
    let progress_ref = progress.clone();
    tokio::spawn(async move {
        let mut progress_guard = progress_ref.lock().await;
        progress_guard.update().await;
    });

    match home.current_screen {
        ScreenState::Sessions => {
            let f_mutex_clone = Arc::clone(&f_mutex);
            tokio::spawn(async move {
                let mut frame_guard = f_mutex_clone.lock().await;
                let frame: &mut ratatui::Frame<'_> = frame_guard.deref_mut();
                draw_session_table_ui(frame, table, home);
                if table.help {
                    help.draw_dashboard_help(frame);
                }
            });
        }
        ScreenState::Transfer => {
            let f_mutex_clone = Arc::clone(&f_mutex);
            tokio::spawn(async move {
                let mut frame_guard = f_mutex_clone.lock().await;
                let frame: &mut ratatui::Frame<'_> = frame_guard.deref_mut();
                table_ui(frame, table);
                if table.help {
                    help.draw_dashboard_help(frame);
                }
            });
        }
        ScreenState::Connection => {
            let f_mutex_clone = Arc::clone(&f_mutex);
            tokio::spawn(async move {
                let mut frame_guard = f_mutex_clone.lock().await;
                let frame: &mut ratatui::Frame<'_> = frame_guard.deref_mut();
                table.active = false;
                if connection.visible {
                    connection.render(frame);
                }
            });
        }
        ScreenState::TCP => {
            let f_mutex_clone = Arc::clone(&f_mutex);
            tokio::spawn(async move {
                let mut frame_guard = f_mutex_clone.lock().await;
                let frame: &mut ratatui::Frame<'_> = frame_guard.deref_mut();
                connection.visible = false;
                table.active = false;
                if host.visible {
                    host.render(frame);
                }
            });
        }
        ScreenState::TcpServer => {
            let progress_ref = progress.clone();
            let f_mutex_clone = Arc::clone(&f_mutex);
            if connection.logs {
                tokio::spawn(async move {
                    let mut progress_guard = progress_ref.lock().await;
                    let mut frame_guard = f_mutex_clone.lock().await;
                    let frame: &mut ratatui::Frame<'_> = frame_guard.deref_mut();
                    progress_guard.draw(frame);
                });
            }
        }
        ScreenState::TcpClient => {
            let f_mutex_clone = Arc::clone(&f_mutex);
            tokio::spawn(async move {
                let mut frame_guard = f_mutex_clone.lock().await;
                let frame: &mut ratatui::Frame<'_> = frame_guard.deref_mut();
                if connection.input_popup {
                    connection.draw_input(frame, connection.returned_val, input_box);
                }
            });
        }
        _ => {}
    }
}
