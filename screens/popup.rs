use crate::home::homepage::Home;
use ratatui::style::{Style, Stylize};
use ratatui::text::Span;
use ratatui::widgets::Borders;
use ratatui::{layout::Rect, text::Line, widgets::Clear, Frame};
use tui_confirm_dialog::{ButtonLabel, ConfirmDialog, ConfirmDialogState};

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
    pub fn render_text_area() {}
}
