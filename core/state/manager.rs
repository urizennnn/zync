use super::state::{ScreenState, StateSnapshot};
use crate::screens::{dashboard::table_ui, home::Home, session::draw_session_table_ui};
use ratatui::DefaultTerminal;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn manage_state(
    home: &mut Home,
    state_snapshot: Arc<StateSnapshot>,
    term: Arc<Mutex<DefaultTerminal>>,
) -> Result<(), Box<dyn Error>> {
    let current_screen = home.current_screen.clone();
    let state_clone = Arc::clone(&state_snapshot);
    let term_clone = Arc::clone(&term);
    let mut handles = Vec::new();

    if current_screen == ScreenState::TcpServer {
        let state_tcp = Arc::clone(&state_snapshot);
        let term_tcp = Arc::clone(&term);
        let handle = thread::spawn(move || -> Result<(), std::io::Error> {
            let progress = state_tcp.progress.lock().unwrap();
            let mut terminal = term_tcp.lock().unwrap();
            terminal.draw(|f| {
                progress.draw(f);
            })?;
            Ok(())
        });
        handles.push(handle);
    }

    let handle = thread::spawn(move || -> Result<(), std::io::Error> {
        let mut terminal = term_clone.lock().unwrap();
        terminal.draw(|frame| match current_screen {
            ScreenState::Sessions => {
                let mut table = state_clone.table.lock().unwrap();
                let mut help = state_clone.help.lock().unwrap();
                draw_session_table_ui(frame, &mut table);
                if table.help {
                    help.draw_dashboard_help(frame);
                }
            }
            ScreenState::Transfer => {
                let mut table = state_clone.table.lock().unwrap();
                let mut help = state_clone.help.lock().unwrap();
                table_ui(frame, &mut table);
                if table.help {
                    help.draw_dashboard_help(frame);
                }
            }
            ScreenState::Connection => {
                let connection = state_clone.connection.lock().unwrap();
                let mut table = state_clone.table.lock().unwrap();
                table.active = false;

                if connection.visible {
                    connection.render(frame);
                }
            }
            ScreenState::TCP => {
                let host = state_clone.host.lock().unwrap();
                let mut table = state_clone.table.lock().unwrap();
                table.active = false;
                if host.visible {
                    host.render(frame);
                }
            }
            _ => {}
        })?;
        Ok(())
    });
    handles.push(handle);

    for handle in handles {
        handle.join().map_err(|_| "Thread panicked")??;
    }

    Ok(())
}
