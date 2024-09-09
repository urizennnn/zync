pub mod hello {
    use ratatui::{
        layout::{Constraint, Layout},
        style::{Style, Stylize},
        widgets::{Block, Paragraph},
        DefaultTerminal,
    };
    use std::{error::Error, io};
    use tui_big_text::{BigText, BigTextBuilder, PixelSize};

    use crossterm::event::{self, Event, KeyEventKind};
    use ratatui::{
        layout::Rect,
        style::Modifier,
        text::{Line, Span},
        widgets::Widget,
    };

    pub struct Home {
        running: bool,
    }

    impl Home {
        pub fn handle_events(&mut self) -> io::Result<()> {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == event::KeyCode::Char('q') {
                    self.running = false;
                }
            }
            Ok(())
        }

        pub fn run(mut self, mut term: DefaultTerminal) -> Result<(), Box<dyn Error>> {
            while self.running {
                term.draw(|f| f.render_widget(&self, f.area())).unwrap();
                self.handle_events().unwrap();
            }
            Ok(())
        }

        /// Function to create a `BigText` object for large text rendering.
        fn create_big_text() -> BigText<'static> {
            BigTextBuilder::default()
                .pixel_size(PixelSize::Quadrant) // Correct usage
                .lines(["TCSHARE".into()]) // Use a single string for the text
                .style(Style::default().fg(ratatui::style::Color::Red)) // Apply the style to BigTextBuilder
                .build() // Build the BigText with the configured properties
        }

        /// Function to create regular text as `Line`.
        fn create_line() -> Vec<Line<'static>> {
            let line = "Welcome to TCSHARE. Hit n to start your new file sharing session.";
            let styled_text = Span::styled(line, Style::default().add_modifier(Modifier::BOLD));
            vec![Line::from(styled_text)]
        }

        fn create_title(title: &str) -> Block {
            Block::default()
                .title(Span::styled(
                    title,
                    Style::default().add_modifier(Modifier::BOLD),
                ))
                .borders(ratatui::widgets::Borders::ALL)
        }

        pub fn new() -> Self {
            Self { running: true }
        }
    }

    impl Default for Home {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Widget for &Home {
        fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
            // Layout for different sections
            let chunks = Layout::default()
                .constraints([Constraint::Max(10), Constraint::Min(3)].as_ref())
                .split(area);

            // Render BigText separately
            let big_text = Home::create_big_text();
            big_text.render(chunks[0], buf); // Render BigText in the first chunk

            // Render regular text
            let paragraph = Paragraph::new(Home::create_line())
                .block(Home::create_title("Welcome"))
                .style(Style::default().fg(ratatui::style::Color::Gray));

            paragraph.render(chunks[1], buf); // Render normal text in the second chunk
        }
    }
}
