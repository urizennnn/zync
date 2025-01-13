use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Tabs},
    Frame,
};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

use crate::{
    screens::popup::InputBox,
    state::state::ScreenState,
    utils::calculate::{calculate_popup_area, centered_rect},
};

#[derive(Debug, Clone, Copy, PartialEq, FromRepr, Display, EnumIter)]
#[repr(usize)]
pub enum ConnectionType {
    TCP,
    P2P,
    WIFI,
}

#[derive(Debug)]
pub struct ConnectionPopup {
    pub returned_val: Option<ConnectionType>,
    pub selected: ConnectionType,
    pub visible: bool,
    pub logs: bool,
    pub input_popup: bool,
}

impl ConnectionType {
    pub fn next_val(self) -> Self {
        let index: usize = self as usize;
        let next_index = index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }
    pub fn previous_val(self) -> Self {
        let index: usize = self as usize;
        let next_index = index.saturating_sub(1);
        Self::from_repr(next_index).unwrap_or(self)
    }
    pub fn return_selected_type(&self) -> ConnectionType {
        *self
    }
}
impl ConnectionPopup {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            returned_val: None,
            logs: false,
            input_popup: false,
            selected: ConnectionType::TCP,
            visible: true,
        }
    }

    pub fn render(&self, f: &mut Frame) {
        let area = f.area();
        let popup_area = centered_rect(30, 20, area);

        f.render_widget(Clear, popup_area);

        let popup_block = Block::default()
            .title("Choose Connection Type")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));

        f.render_widget(popup_block, popup_area);

        let inner_area = popup_area.inner(ratatui::layout::Margin {
            horizontal: (2),
            vertical: (2),
        });
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(2), // Question
                    Constraint::Length(1), // Spacing
                    Constraint::Length(3), // Options
                    Constraint::Length(2), // Instructions
                ]
                .as_ref(),
            )
            .split(inner_area);

        let question = Paragraph::new(Line::from(vec![Span::styled(
            "How would you like to connect?",
            Style::default().fg(Color::White),
        )]))
        .alignment(Alignment::Center);
        f.render_widget(question, chunks[0]);

        let titles = ConnectionType::iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>();
        let tabs = Tabs::new(titles)
            .select(self.selected as usize)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan))
            .divider("|");
        f.render_widget(tabs, chunks[2]);

        let instructions = Paragraph::new(Line::from(vec![Span::styled(
            "← → to select • Enter to confirm • n to cancel",
            Style::default().fg(Color::DarkGray),
        )]))
        .alignment(Alignment::Center);
        f.render_widget(instructions, chunks[3]);
    }

    pub fn next(&mut self) {
        self.selected = self.selected.next_val();
    }

    pub fn previous(&mut self) {
        self.selected = self.selected.previous_val();
    }
    pub fn return_selected(&mut self) -> ConnectionType {
        self.returned_val = Some(self.selected);
        self.selected.return_selected_type()
    }

    pub fn draw_input(
        &mut self,
        f: &mut Frame,
        screen_state: Option<ConnectionType>,
        input: &mut InputBox,
    ) {
        match screen_state {
            Some(ConnectionType::TCP) => {
                let area = calculate_popup_area(f.area(), 25, 20);

                f.render_widget(Clear, area);

                let block = Block::default()
                    .title("TCP IP Address")
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
                        Constraint::Length(3),
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                    ])
                    .split(inner_area);

                let prompt =
                    Paragraph::new("Enter IP Address:").style(Style::default().fg(Color::White));
                f.render_widget(prompt, text_area[0]);
                input.draw_in_popup(f, text_area[1]);

                let instructions = Paragraph::new(Line::from(vec![Span::styled(
                    "Hit `e` to start typing • Enter to submit • q to cancel",
                    Style::default().fg(Color::DarkGray),
                )]))
                .alignment(Alignment::Center);
                f.render_widget(instructions, text_area[4]);
            }
            _ => {
                todo!("Not implemented for {:?}", screen_state)
            }
        }
    }

    pub fn draw_connection_logs(&mut self, f: &mut Frame, scrren_state: ScreenState) {
        match scrren_state {
            ScreenState::Connection => {
                let area = calculate_popup_area(f.area(), 25, 20);

                f.render_widget(Clear, area);

                let block = Block::default()
                    .title("Connection Logs")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan));

                f.render_widget(block.clone(), area);

                let inner_area = area.inner(ratatui::layout::Margin {
                    vertical: 1,
                    horizontal: 2,
                });

                // Render prompt text
                let text_area = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(1), // For prompt
                        Constraint::Length(3), // For input box
                        Constraint::Length(1), // For Spacej
                        Constraint::Length(1), // For Spacej
                        Constraint::Length(1), // For instructions
                    ])
                    .split(inner_area);

                let prompt =
                    Paragraph::new("Enter IP Address:").style(Style::default().fg(Color::White));
                f.render_widget(prompt, text_area[0]);
                let instructions = Paragraph::new(Line::from(vec![Span::styled(
                    "Hit `e` to start typing • Enter to submit • q to cancel",
                    Style::default().fg(Color::DarkGray),
                )]))
                .alignment(Alignment::Center);
                f.render_widget(instructions, text_area[4]);
            }
            _ => {
                todo!("Not implemented for {:?}", scrren_state)
            }
        }
    }
}
