pub mod homepage {
    use ratatui::{
        layout::{Constraint, Direction, Layout, Rect},
        style::{Color, Modifier, Style},
        text::{Line, Span},
        widgets::{Block, BorderType, Borders, Paragraph, Widget},
        Frame,
    };
    use std::{error::Error, io};
    use tui_big_text::{BigText, BigTextBuilder, PixelSize};
    use tui_confirm_dialog::{ButtonLabel, ConfirmDialog, ConfirmDialogState, Listener};

    use crossterm::event::{self, Event, KeyCode, KeyEventKind};

    #[derive(Debug)]
    pub struct Home {
        running: bool,
        selected_button: usize,
        confirm_popup: ConfirmDialogState,
        popup_tx: std::sync::mpsc::Sender<Listener>,
        popup_rx: std::sync::mpsc::Receiver<Listener>,
        close_status: Option<String>,
    }

    impl Home {
        pub fn new() -> Self {
            let (tx, rx) = std::sync::mpsc::channel();
            Self {
                running: true,
                selected_button: 0,
                confirm_popup: ConfirmDialogState::default(),
                popup_tx: tx,
                popup_rx: rx,
                close_status: None,
            }
        }

        pub fn run(
            &mut self,
            mut term: ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
        ) -> Result<(), Box<dyn Error>> {
            while self.running {
                term.draw(|f| self.ui(f))?;
                self.handle_events()?;
            }
            Ok(())
        }

        fn handle_events(&mut self) -> io::Result<()> {
            if let Ok(res) = self.popup_rx.try_recv() {
                if res.0 == self.confirm_popup.id {
                    self.close_status = Some(format!("Dialog closed with result: {:?}", res.1));
                    self.confirm_popup = ConfirmDialogState::default();
                }
            }

            if let Event::Key(key) = event::read()? {
                if self.confirm_popup.is_opened() && self.confirm_popup.handle(key) {
                    return Ok(());
                }

                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => self.running = false,
                        KeyCode::Char('n') => self.open_confirm_dialog(),
                        KeyCode::Esc => self.confirm_popup = ConfirmDialogState::default(),
                        KeyCode::Right | KeyCode::Left if self.confirm_popup.is_opened() => {
                            self.selected_button = 1 - self.selected_button;
                        }
                        _ => {}
                    }
                }
            }
            Ok(())
        }

        fn open_confirm_dialog(&mut self) {
            self.confirm_popup = ConfirmDialogState::default()
                .modal(false)
                .with_title("Confirmation")
                .with_text(vec![
                    Line::from("Are you an admin?"),
                    Line::from("This will grant you additional privileges."),
                ])
                .with_yes_button(ButtonLabel::from("(Y)es").unwrap())
                .with_no_button(ButtonLabel::from("(N)o").unwrap())
                .with_yes_button_selected(false)
                .with_listener(Some(self.popup_tx.clone()));
            self.confirm_popup = self.confirm_popup.open();
        }

        fn ui(&mut self, f: &mut Frame) {
            let area = f.size();

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(30),
                    Constraint::Min(10),
                    Constraint::Length(1),
                ])
                .split(area);

            self.render_content(f, layout);

            if self.confirm_popup.is_opened() {
                let popup = ConfirmDialog::default()
                    .borders(Borders::ALL)
                    .bg(Color::Black)
                    .border_type(BorderType::Rounded)
                    .button_style(Style::default())
                    .selected_button_style(Style::default().yellow().underlined().bold());
                f.render_stateful_widget(popup, area, &mut self.confirm_popup);
            }
        }

        fn render_content(&self, f: &mut Frame, layout: Vec<Rect>) {
            let border = Self::create_border();
            f.render_widget(border.clone(), layout[0]);

            let (big_text, normal_text) = Self::create_big_text();

            let content_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(5), Constraint::Length(2)])
                .split(layout[1]);

            let big_text_width = 30;
            let big_text_area = Rect {
                x: layout[1].width.saturating_sub(big_text_width) / 2,
                y: content_layout[0].y,
                width: big_text_width,
                height: content_layout[0].height,
            };
            f.render_widget(big_text, big_text_area);

            let paragraph =
                Paragraph::new(normal_text).alignment(ratatui::layout::Alignment::Center);
            f.render_widget(paragraph, content_layout[1]);

            Self::draw_commands(layout[2], f);
        }

        fn create_big_text() -> (BigText<'static>, Vec<Line<'static>>) {
            let text = BigTextBuilder::default()
                .pixel_size(PixelSize::Quadrant)
                .lines(["TCSHARE".into()])
                .style(Style::default().fg(Color::Red))
                .build();
            let line = Self::create_line();
            (text, line)
        }

        fn create_line() -> Vec<Line<'static>> {
            let line = "Welcome to TCSHARE. Hit n to start your new file sharing session.";
            let styled_text = Span::styled(line, Style::default().add_modifier(Modifier::BOLD));
            vec![Line::from(styled_text)]
        }

        fn create_border() -> Block<'static> {
            Block::default()
                .border_type(BorderType::Rounded)
                .borders(Borders::TOP)
                .title(Line::from("Welcome").centered())
        }

        fn draw_commands(area: Rect, f: &mut Frame) {
            let command_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(10), Constraint::Min(20)])
                .split(area);

            let label = Paragraph::new("Commands")
                .style(Style::default().add_modifier(Modifier::BOLD))
                .alignment(ratatui::layout::Alignment::Left);
            f.render_widget(label, command_layout[0]);

            let commands_text = "q: Quit | n: Start a new file sharing session";
            let commands =
                Paragraph::new(commands_text).alignment(ratatui::layout::Alignment::Right);
            f.render_widget(commands, command_layout[1]);
        }
    }

    impl Default for Home {
        fn default() -> Self {
            Self::new()
        }
    }
}
