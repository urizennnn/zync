use crate::screens::popup::InputBox;
use crate::utils::calculate::{calculate_popup_area, centered_rect};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Tabs};
use strum::IntoEnumIterator;
use strum::{Display, EnumIter, FromRepr};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionInputMode {
    Server,
    Client,
}

#[derive(Debug)]
pub struct ConnectionPopup {
    pub returned_val: Option<ConnectionType>,
    pub selected: ConnectionType,
    pub visible: bool,
    pub logs: bool,
    pub input_popup: bool,
    pub mode: ConnectionInputMode,
    pub log: String,
}

#[derive(Debug, Clone, Copy, PartialEq, FromRepr, Display, EnumIter)]
#[repr(usize)]
pub enum ConnectionType {
    TCP,
    P2P,
    WIFI,
}

impl ConnectionType {
    pub fn next_val(self) -> Self {
        let index = self as usize;
        let next_index = index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }
    pub fn previous_val(self) -> Self {
        let index = self as usize;
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
            log: String::new(),
            returned_val: None,
            logs: false,
            input_popup: false,
            selected: ConnectionType::TCP,
            visible: true,
            mode: ConnectionInputMode::Client,
        }
    }
    pub fn previous(&mut self) {
        self.selected = self.selected.previous_val();
    }
    pub fn next(&mut self) {
        self.selected = self.selected.next_val();
    }
    pub fn return_selected(&mut self) -> Option<ConnectionType> {
        self.returned_val = Some(self.selected.return_selected_type());
        self.returned_val
    }

    pub fn render(&self, f: &mut Frame) {
        let area = f.area();
        let popup_area = calculate_popup_area(area, 30, 20);
        f.render_widget(Clear, popup_area);
        let popup_block = Block::default()
            .title("Choose Connection Type")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));
        f.render_widget(popup_block, popup_area);
        let inner_area = popup_area.inner(ratatui::layout::Margin {
            horizontal: 2,
            vertical: 2,
        });
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(2),
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(2),
            ])
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

    pub fn draw_input(&mut self, f: &mut Frame, input: &mut InputBox) {
        let area = centered_rect(30, 20, f.area());
        f.render_widget(Clear, area);

        let (title, prompt_text) = match self.mode {
            ConnectionInputMode::Server => ("Port Number", "Enter port number:"),
            ConnectionInputMode::Client => ("TCP IP Address", "Enter IP Address:"),
        };

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .style(Style::default().fg(Color::LightYellow));
        f.render_widget(block.clone(), area);

        let inner_area = area.inner(ratatui::layout::Margin {
            vertical: 1,
            horizontal: 2,
        });
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // prompt
                Constraint::Length(3), // input box
                Constraint::Length(1), // spacer
                Constraint::Length(1), // spacer
                Constraint::Length(1), // instructions
            ])
            .split(inner_area);

        let prompt = Paragraph::new(prompt_text).style(Style::default().fg(Color::White));
        f.render_widget(prompt, chunks[0]);

        input.draw_in_popup(f, chunks[1]);

        let instructions = Paragraph::new("Hit e to start typing • Enter to submit • q to cancel")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(instructions, chunks[4]);
    }
}
