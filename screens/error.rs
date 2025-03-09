pub mod error_widget {
    use ratatui::{
        Frame,
        layout::{Alignment, Rect},
        style::{Color, Style},
        text::Line,
        widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    };

    use crate::utils::calculate::calculate_popup_area;

    #[derive(Debug)]
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
                message: "An error occurred".to_string(),
                title: Line::from(" Error").style(Style::default().fg(Color::Red)),
                button: "Ok".to_string(),
            }
        }

        /// Set the text/title/color for this error message
        pub fn set_val(&mut self, message: String, level: &mut ErrorType, button: String) {
            self.message = message;
            self.button = button;
            match level {
                ErrorType::Error => {
                    self.title = Line::from(" Error").style(Style::default().fg(Color::Red));
                }
                ErrorType::Warning => {
                    self.title = Line::from(" Warning").style(Style::default().fg(Color::Yellow));
                }
                ErrorType::Info => {
                    self.title = Line::from(" Info").style(Style::default().fg(Color::Blue));
                }
            }
        }

        /// Render as a centered popup (the old behavior)
        pub fn render_centered_popup(&self, f: &mut Frame) {
            let message_lines = vec![self.title.clone(), Line::from(self.message.clone())];
            let paragraph = Paragraph::new(message_lines)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::default().fg(Color::Blue)),
                )
                .wrap(Wrap { trim: true })
                .alignment(Alignment::Center);

            let area = calculate_popup_area(f.area(), 30, 10);
            f.render_widget(Clear, area);
            f.render_widget(paragraph, area);
        }

        /// Render in the top-right corner (small “toast” style)
        ///
        /// * `width` and `height` define the box size
        pub fn render_in_corner(&self, f: &mut Frame, width: u16, height: u16) {
            // Calculate top-right corner area
            let screen = f.area();
            let x = screen.x + screen.width.saturating_sub(width);
            let y = screen.y; // top edge
            let area = Rect::new(x, y, width, height);

            let message_lines = vec![self.title.clone(), Line::from(self.message.clone())];
            let paragraph = Paragraph::new(message_lines)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::default().fg(Color::Blue)),
                )
                .wrap(Wrap { trim: true })
                .alignment(Alignment::Left);

            f.render_widget(Clear, area);
            f.render_widget(paragraph, area);
        }
    }
}
