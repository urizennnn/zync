pub mod hello {
    use ratatui::{
        layout::{Constraint, Layout},
        style::{Style, Stylize},
        widgets::{Block, Paragraph},
        DefaultTerminal,
    };
    use std::{error::Error, io};

    use crossterm::event::{self, Event, KeyEventKind};
    use ratatui::{
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

        pub fn new() -> Self {
            Self { running: true }
        }

        pub fn run(mut self, mut term: DefaultTerminal) -> Result<(), Box<dyn Error>> {
            while self.running {
                term.draw(|f| f.render_widget(&self, f.area())).unwrap();
                self.handle_events().unwrap();
            }
            Ok(())
        }

        fn create_line() -> Vec<Line<'static>> {
            let line = "Welcome to TCSHARE. Hit n to start your new file sharing session.";
            let spanned_text = Span::styled(line, Style::default().add_modifier(Modifier::BOLD));
            vec![Line::from(spanned_text)]
        }

        fn create_title(title: &str) -> Block {
            Block::bordered()
                .gray()
                .title(title.bold().into_centered_line())
        }
    }

    impl Widget for &Home {
        fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
            let area = Layout::vertical([Constraint::Max(9); 4]).split(area);
            Paragraph::new(self::Home::create_line())
                .block(Home::create_title("Welcome"))
                .gray()
                .centered()
                .render(area[2], buf)
        }
    }
}
