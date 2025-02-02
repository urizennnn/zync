use crate::{
    core_mod::event::{AsyncEvent, NewTrait},
    state::state::ConnectionState,
    utils::calculate::calculate_popup_area,
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

#[derive(Debug)]
pub struct ConnectionProgress {
    state: ConnectionState,
    event: AsyncEvent<ConnectionState>,
}

impl Default for ConnectionProgress {
    fn default() -> Self {
        Self::new()
    }
}

impl ConnectionProgress {
    pub fn new() -> Self {
        Self {
            state: ConnectionState::Connecting,
            event: AsyncEvent::new(),
        }
    }

    pub async fn update(&mut self) {
        if let Some(new_state) = self.event.recv().await {
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
        };

        let paragraph = Paragraph::new(message).style(style);
        f.render_widget(paragraph, text_area[0]);
    }

    pub fn get_event_sender(&self) -> &AsyncEvent<ConnectionState> {
        &self.event
    }
}

// Example usage in a connection handler
async fn handle_connection(event: &AsyncEvent<ConnectionState>) {
    // Update connection states
    event.send(ConnectionState::Connecting).await;

    // Simulate some connection work
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Check connection result
    match /* connection result */ true {
        true => event.send(ConnectionState::Connected).await,
        false => event.send(ConnectionState::Failed("Connection failed".to_string())).await,
    }
}
