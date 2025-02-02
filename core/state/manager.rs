use super::state::{ScreenState, StateSnapshot};
use crate::screens::{dashboard::table_ui, home::Home, session::draw_session_table_ui};
use ratatui::DefaultTerminal;
use std::error::Error;
use std::sync::{Arc, Mutex};

pub fn manage_state(
    home: &mut Home,
    state_snapshot: Arc<StateSnapshot>,
    term: Arc<Mutex<DefaultTerminal>>,
) -> Result<(), Box<dyn Error>> {
    let current_screen = home.current_screen.clone();
    let mut terminal = term.lock().unwrap();

    terminal.draw(|frame| {
        if current_screen == ScreenState::TcpServer {
            let progress = state_snapshot.progress.lock().unwrap();
            progress.draw(frame);
        }

        match current_screen {
            ScreenState::Sessions => {
                let mut table = state_snapshot.table.lock().unwrap();
                let mut help = state_snapshot.help.lock().unwrap();
                draw_session_table_ui(frame, &mut table);
                if table.help {
                    help.draw_dashboard_help(frame);
                }
            }
            ScreenState::Transfer => {
                let mut table = state_snapshot.table.lock().unwrap();
                let mut help = state_snapshot.help.lock().unwrap();
                table_ui(frame, &mut table);
                if table.help {
                    help.draw_dashboard_help(frame);
                }
            }
            ScreenState::Connection => {
                let connection = state_snapshot.connection.lock().unwrap();
                let mut table = state_snapshot.table.lock().unwrap();
                table.active = false;
                if connection.visible {
                    connection.render(frame);
                }
            }
            ScreenState::TCP => {
                let host = state_snapshot.host.lock().unwrap();
                let mut table = state_snapshot.table.lock().unwrap();
                table.active = false;
                if host.visible {
                    host.render(frame);
                }
            }
            _ => {}
        }
    })?;

    Ok(())
}
