pub mod error_widget {
    use ratatui::{
        layout::Alignment,
        style::Style,
        text::Line,
        widgets::{Clear, Paragraph},
        Frame,
    };

    use crate::calculate::calculate_popup_area;

    pub struct ErrorWidget {
        pub message: String,
        pub title: Line<'static>,
        pub button: String,
    }

    pub enum ErrorType {
        Error,
        Warning,
        Info,
    }

    impl ErrorWidget {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self {
                message: "An error occured".to_string(),
                title: Line::from(" Error").style(Style::default().fg(ratatui::style::Color::Red)),
                button: "Ok".to_string(),
            }
        }
        pub fn set_val(&mut self, message: String, level: &mut ErrorType, button: String) {
            self.message = message;
            self.button = button;
            match level {
                ErrorType::Error => {
                    self.title = Line::from(" Error")
                        .style(Style::default().fg(ratatui::style::Color::Red));
                }
                ErrorType::Warning => {
                    self.title = Line::from(" Warning")
                        .style(Style::default().fg(ratatui::style::Color::Yellow));
                }
                ErrorType::Info => {
                    self.title = Line::from(" Info")
                        .style(Style::default().fg(ratatui::style::Color::Blue));
                }
            }
        }

        pub fn render_popup(&mut self, f: &mut Frame) {
            let message = vec![self.title.clone(), Line::from(self.message.clone())];
            let pg = Paragraph::new(message)
                .block(
                    ratatui::widgets::Block::default()
                        .borders(ratatui::widgets::Borders::ALL)
                        .border_style(Style::default().fg(ratatui::style::Color::Blue))
                        .border_type(ratatui::widgets::BorderType::Rounded),
                )
                .wrap(ratatui::widgets::Wrap { trim: { true } })
                .alignment(Alignment::Center);
            let area = calculate_popup_area(f.area(), 20, 10);
            f.render_widget(Clear, area);
            f.render_widget(pg, area);
        }
    }
}
