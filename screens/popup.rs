use std::error::Error;

use crate::home::homepage::Home;
use derive_setters::Setters;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Span, Text};
use ratatui::widgets::{Block, Borders, Paragraph, Widget, Wrap};
use ratatui::{layout::Rect, text::Line, widgets::Clear, Frame};
use tui_confirm_dialog::{ButtonLabel, ConfirmDialog, ConfirmDialogState};
use tui_textarea::TextArea;

#[derive(Default, Setters)]
pub struct ApiPopup<'a> {
    pub title: String,
    pub message: String,
    pub input: Block<'a>,
    pub buttons: Vec<String>,
}

impl Widget for ApiPopup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Create the block that will encompass the entire popup
        let outer_block = Block::default()
            .title(self.title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ratatui::style::Color::Yellow));

        // Render the outer block, which encompasses everything (message and input)
        outer_block.render(area, buf);

        // Create layout with two vertical chunks: one for the message, one for the input
        let chunks = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Percentage(50), // The message will take the upper 50% of the block
                Constraint::Percentage(50), // The input will take the lower 50% of the block
            ])
            .split(area);

        // Center the message in the upper chunk
        let centered_message = Paragraph::new(self.message)
            .alignment(ratatui::layout::Alignment::Center)
            .wrap(Wrap { trim: true });

        // Render the centered message in the upper chunk
        centered_message.render(chunks[0], buf);

        // Render the input area in the lower chunk
        let mut input = TextArea::default();
        input.set_block(Block::default().borders(Borders::ALL)); // Set input block with border
        input.render(chunks[1], buf);
    }
}

impl<'a> ApiPopup<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Home {
    pub fn render_notification(&mut self, frame: &mut Frame) {
        self.popup_dialog = ConfirmDialogState::default()
            .modal(true)
            .with_title(Span::styled("Notification", Style::new().bold().cyan()))
            .with_text(vec![Line::from("Are you an admin?")])
            .with_yes_button(ButtonLabel::from("(Y)es").unwrap())
            .with_no_button(ButtonLabel::from("(N)o").unwrap())
            .with_yes_button_selected(self.selected_button == 0)
            .with_listener(Some(self.popup_tx.clone()))
            .open();

        let area = self.calculate_popup_area(frame.area(), 50, 30);

        if self.popup_dialog.is_opened() {
            let popup = ConfirmDialog::default()
                .borders(Borders::ALL)
                .bg(ratatui::style::Color::Black)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .button_style(ratatui::prelude::Style::default()) // Default button style
                .selected_button_style(
                    ratatui::prelude::Style::default()
                        .fg(ratatui::style::Color::Yellow)
                        .bold(),
                );

            frame.render_widget(Clear, area);

            frame.render_stateful_widget(popup, area, &mut self.popup_dialog);
        }
    }

    fn calculate_popup_area(&self, area: Rect, percent_x: u16, percent_y: u16) -> Rect {
        let popup_width = area.width * percent_x / 100;
        let popup_height = area.height * percent_y / 100;

        let popup_x = (area.width - popup_width) / 2;
        let popup_y = (area.height - popup_height) / 2;

        Rect::new(
            area.x + popup_x,
            area.y + popup_y,
            popup_width,
            popup_height,
        )
    }
    pub fn show_api_popup(&mut self, frame: &mut Frame) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
