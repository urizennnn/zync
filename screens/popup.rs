use crate::home::homepage::Home;
use ratatui::widgets::Borders;
use ratatui::{layout::Rect, text::Line, widgets::Clear, Frame};
use tui_confirm_dialog::{ButtonLabel, ConfirmDialog, ConfirmDialogState};

impl Home {
    pub fn render_popup(&mut self, frame: &mut Frame) {
        // Set up the popup dialog
        self.popup_dialog = ConfirmDialogState::default()
            .modal(false)
            .with_title("Notification")
            .with_text(vec![Line::from("Are you an admin?")])
            .with_yes_button(ButtonLabel::from("(Y)es").unwrap())
            .with_no_button(ButtonLabel::from("(N)o").unwrap())
            .with_listener(Some(self.popup_tx.clone()))
            .open();

        // Define popup area (optional, if you want to calculate custom dimensions)
        let area = self.calculate_popup_area(frame.area(), 50, 30);
        if self.popup_dialog.is_opened() {
            let popup = ConfirmDialog::default()
                .borders(Borders::ALL)
                .bg(ratatui::style::Color::DarkGray)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .button_style(ratatui::prelude::Style::default())
                .selected_button_style(
                    ratatui::prelude::Style::default().fg(ratatui::style::Color::Yellow),
                );

            // Clear the area behind the popup
            frame.render_widget(Clear, area);

            // Render the popup dialog widget
            frame.render_stateful_widget(popup, area, &mut self.popup_dialog);
        }
    }

    #[allow(dead_code)]
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
}
