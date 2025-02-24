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
    home.spawn_upload_handler_if_needed();
    let current_screen = home.current_screen.clone();
    let mut terminal = term.lock().unwrap();
    terminal.draw(|frame| match current_screen {
        ScreenState::Sessions => {
            let mut help = state_snapshot.help.lock().unwrap();
            let mut table = state_snapshot.table.lock().unwrap();
            let progress = state_snapshot.progress.clone();
            draw_session_table_ui(frame, &mut table, progress);
            if table.help {
                help.draw_dashboard_help(frame);
            }
        }
        ScreenState::Transfer => {
            let mut table = state_snapshot.table.lock().unwrap();
            let progress = state_snapshot.progress.clone();
            let mut help = state_snapshot.help.lock().unwrap();
            table_ui(frame, &mut table, progress);
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
        ScreenState::Debug => {
            let debug = state_snapshot.debug_screen.lock().unwrap();
            debug.draw(frame, frame.area());
        }
        ScreenState::TcpClient => {
            let mut input = state_snapshot.connection.lock().unwrap();
            let mut table = state_snapshot.table.lock().unwrap();
            let mut input_box = state_snapshot.input_box.lock().unwrap();
            input.mode = crate::screens::protocol_popup::ConnectionInputMode::Client;
            table.active = false;
            input.draw_input(frame, &mut input_box);
        }
        ScreenState::TcpServer => {
            let mut input = state_snapshot.connection.lock().unwrap();
            let mut table = state_snapshot.table.lock().unwrap();
            let mut input_box = state_snapshot.input_box.lock().unwrap();
            input.mode = crate::screens::protocol_popup::ConnectionInputMode::Server;
            table.active = false;
            input.draw_input(frame, &mut input_box);
        }
        _ => {}
    })?;
    Ok(())
}
