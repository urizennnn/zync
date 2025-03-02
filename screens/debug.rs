use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Borders, Clear, Paragraph},
};

#[derive(Debug)]
pub struct DebugScreen {
    pub lines: Vec<String>,
}

impl DebugScreen {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }

    pub fn push_line(&mut self, line: impl Into<String>) {
        self.lines.push(line.into());
    }

    pub fn draw(&self, f: &mut Frame, area: Rect) {
        f.render_widget(Clear, area);
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Debugger")
            .border_style(Style::default().fg(Color::Yellow));
        f.render_widget(block, area);

        let lines: Vec<Line> = self
            .lines
            .iter()
            .map(|content| Line::from(content.clone()))
            .collect();
        let paragraph = Paragraph::new(Text::from(lines))
            .alignment(Alignment::Left)
            .wrap(ratatui::widgets::Wrap { trim: true });
        let inner = area.inner(ratatui::layout::Margin {
            horizontal: 1,
            vertical: 1,
        });
        f.render_widget(paragraph, inner);
    }
}
