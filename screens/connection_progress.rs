use crate::{
    core_mod::event::{SyncEvent, SyncTrait},
    state::state::ConnectionState,
    utils::calculate::calculate_popup_area,
};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
};

#[derive(Debug)]
pub struct ConnectionProgress {
    pub state: ConnectionState,
    event: SyncEvent<ConnectionState>,
}

impl Default for ConnectionProgress {
    fn default() -> Self {
        Self::new()
    }
}

impl ConnectionProgress {
    pub fn new() -> Self {
        Self {
            state: ConnectionState::NoConnection,
            event: SyncEvent::new(),
        }
    }

    pub fn update(&mut self) {
        if let Some(new_state) = self.event.recv() {
            self.state = new_state;
        }
    }

    pub fn draw(&self, f: &mut Frame) {
        let area = calculate_popup_area(f.area(), 25, 20);
        f.render_widget(Clear, f.area());

        let block = Block::default()
            .title("Connection Progress")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));
        f.render_widget(block.clone(), area);

        let inner_area = area.inner(ratatui::layout::Margin {
            vertical: 1,
            horizontal: 2,
        });
        let text_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(inner_area);

        let (message, style): (&str, Style) = match &self.state {
            ConnectionState::Connecting => (
                "Connecting to server...",
                Style::default().fg(Color::Yellow),
            ),
            ConnectionState::Connected => {
                ("Connection established!", Style::default().fg(Color::Green))
            }
            ConnectionState::Failed(err) => (err, Style::default().fg(Color::Red)),
            _ => ("", Style::default().fg(Color::White)),
        };

        let paragraph = Paragraph::new(message).style(style);
        f.render_widget(paragraph, text_area[0]);
    }

    pub fn get_event_sender(&self) -> &SyncEvent<ConnectionState> {
        &self.event
    }
}
