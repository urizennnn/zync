pub mod hello {
    use ratatui::widgets::BorderType;
    use ratatui::widgets::Borders;
    use ratatui::widgets::Paragraph;
    use ratatui::{
        layout::{Constraint, Layout},
        style::Style,
        widgets::Block,
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

        fn create_big_text() -> (BigText<'static>, Vec<Line<'static>>) {
            let text = BigTextBuilder::default()
                .pixel_size(PixelSize::Quadrant)
                .lines(["TCSHARE".into()])
                .style(Style::default().fg(ratatui::style::Color::Red))
                .build();
            let line = Home::create_line();
            (text, line)
        }

        fn create_line() -> Vec<Line<'static>> {
            let line = "Welcome to TCSHARE. Hit n to start your new file sharing session.";
            let styled_text = Span::styled(line, Style::default().add_modifier(Modifier::BOLD));
            vec![Line::from(styled_text)]
        }

        fn create_border() -> Block<'static> {
            Block::new()
                .border_type(BorderType::Rounded)
                .borders(Borders::TOP)
                .title(Line::from("Welcome").centered())
        }

        fn draw_commands(area: Rect, buf: &mut ratatui::prelude::Buffer) {
            let command_layout = Layout::default()
                .direction(ratatui::layout::Direction::Horizontal)
                .constraints([Constraint::Length(10), Constraint::Min(20)])
                .split(area);

            let label = Paragraph::new("Commands")
                .style(Style::default().add_modifier(Modifier::BOLD))
                .alignment(ratatui::layout::Alignment::Left);
            label.render(command_layout[0], buf);

            let commands_text = "q: Quit | n: Start a new file sharing session";
            let commands =
                Paragraph::new(commands_text).alignment(ratatui::layout::Alignment::Right);
            commands.render(command_layout[1], buf);
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
            let layout = Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints([
                    Constraint::Percentage(30),
                    Constraint::Min(10),
                    Constraint::Length(1),
                ])
                .split(area);

            let border = Home::create_border();
            border.clone().render(layout[0], buf);

            let (big_text, normal_text) = Home::create_big_text();

            let content_layout = Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints([Constraint::Length(5), Constraint::Length(2)])
                .split(layout[1]);

            let big_text_width = 30;
            let big_text_area = Rect {
                x: area.width.saturating_sub(big_text_width) / 2,
                y: content_layout[0].y,
                width: big_text_width,
                height: content_layout[0].height,
            };
            big_text.render(big_text_area, buf);

            let paragraph =
                Paragraph::new(normal_text).alignment(ratatui::layout::Alignment::Center);
            paragraph.render(content_layout[1], buf);

            Home::draw_commands(layout[2], buf);
        }
    }
}
