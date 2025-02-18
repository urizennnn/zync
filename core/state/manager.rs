use super::state::{ScreenState, StateSnapshot};
use crate::screens::{dashboard::table_ui, home::Home, session::draw_session_table_ui};
use crate::utils::calculate::calculate_popup_area;
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
    {
        let state_snapshot_clone = Arc::clone(&state_snapshot);
        std::thread::spawn(move || {
            let mut progress = state_snapshot_clone.progress.lock().unwrap();
            progress.update();
        });
    }
    terminal.draw(|frame| match current_screen {
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
        ScreenState::TcpLogs => {
            let area = calculate_popup_area(frame.area(), 30, 20);
            let connection = state_snapshot.connection.lock().unwrap();
            let log_msg = if connection.log.is_empty() {
                "No logs available".to_string()
            } else {
                connection.log.clone()
            };
            let paragraph = ratatui::widgets::Paragraph::new(log_msg)
                .block(
                    ratatui::widgets::Block::default()
                        .borders(ratatui::widgets::Borders::ALL)
                        .border_style(
                            ratatui::style::Style::default().fg(ratatui::style::Color::Yellow),
                        )
                        .title("Connection Logs"),
                )
                .alignment(ratatui::layout::Alignment::Left)
                .wrap(ratatui::widgets::Wrap { trim: true });
            frame.render_widget(paragraph, area);
        }
        _ => {}
    })?;
    Ok(())
}
