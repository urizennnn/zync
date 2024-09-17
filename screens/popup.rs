use ratatui::{layout::Rect, text::Line, Frame};
use tui_confirm_dialog::{ButtonLabel, ConfirmDialogState};

use crate::home::homepage::Home;

impl Home {
    pub fn render_popup(&mut self, _frame: &mut Frame) -> ConfirmDialogState {
        self.popup_dialog = tui_confirm_dialog::ConfirmDialogState::default()
            .modal(false)
            .with_title("Notification")
            .with_text(vec![Line::from("Are you an admin?")])
            .with_yes_button(ButtonLabel::from("(Y)es").unwrap())
            .with_no_button(ButtonLabel::from("(N)o").unwrap())
            .with_listener(Some(self.popup_tx.clone()));

        self.popup_dialog = self.popup_dialog.open();
        self.popup_dialog
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
