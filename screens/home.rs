pub mod homepage {
    use crossterm::event::{self, Event, KeyCode};
    use ratatui::text::{Line, Span};
    use ratatui::DefaultTerminal;
    use ratatui::{
        layout::{Constraint, Layout, Rect},
        style::{Modifier, Style},
        widgets::{Block, Borders, Paragraph, Widget},
    };
    use std::sync::mpsc::{self};
    use std::{error::Error, io};
    use tui_big_text::{BigText, BigTextBuilder};
    use tui_confirm_dialog::{ConfirmDialogState, Listener};

    use crate::core::core_lib::check_config;

    pub struct Home {
        running: bool,
        pub show_popup: bool,
        pub show_api_popup: bool,
        pub show_api_dialog: ConfirmDialogState,
        pub selected_button: usize,
        pub popup_tx: mpsc::Sender<Listener>,
        pub popup_rx: mpsc::Receiver<Listener>,
        pub popup_dialog: ConfirmDialogState,
    }

    impl Home {
        pub fn handle_events(&mut self) -> io::Result<()> {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        if self.show_popup {
                            self.popup_tx
                                .send((self.selected_button as u16, Some(false)))
                                .unwrap();
                            self.show_popup = false;
                        } else {
                            self.running = false;
                        }
                    }
                    KeyCode::Char('n') => {
                        self.show_popup = true;
                    }
                    KeyCode::Esc => {
                        if self.show_popup {
                            self.popup_tx
                                .send((self.selected_button as u16, Some(false)))
                                .unwrap();
                            self.show_popup = false;
                        }
                    }
                    KeyCode::Right => {
                        if self.show_popup {
                            self.selected_button += 1;
                            if self.selected_button == 2 {
                                self.selected_button = 0;
                            }
                            self.popup_tx
                                .send((self.selected_button as u16, None))
                                .unwrap();
                        }
                    }
                    KeyCode::Left => {
                        if self.show_popup {
                            self.selected_button -= 1;
                            if self.selected_button == usize::MAX {
                                self.selected_button = 1;
                            }
                            self.popup_tx
                                .send((self.selected_button as u16, None))
                                .unwrap();
                        }
                    }
                    KeyCode::Enter => {
                        if self.show_popup {
                            self.popup_tx
                                .send((self.selected_button as u16, Some(true)))
                                .unwrap();
                            self.show_popup = false;
                        }
                    }
                    _ => {}
                }
            }
            Ok(())
        }

        pub fn run(mut self, mut term: DefaultTerminal) -> Result<(), Box<dyn Error>> {
            while self.running {
                if let Ok((selected_button, confirmed)) = self.popup_rx.try_recv() {
                    match confirmed {
                        Some(true) => {
                            if selected_button == 0 {
                                self.show_api_popup = true;
                            } else {
                                println!("Confirmed on button No");
                            }
                            self.show_popup = false;
                        }
                        Some(false) => {
                            self.show_popup = false;
                        }
                        None => {}
                    }
                }

                match check_config() {
                    Ok(_) => {
                        todo!("Redirect to dashboard screen");
                    }
                    Err(_) => {
                        term.draw(|f| {
                            let area = f.area();
                            self.render(area, f.buffer_mut());

                            if self.show_popup {
                                self.render_notification(f);
                            }
                            if self.show_api_popup {
                                self.show_api_popup(f).unwrap();
                            }
                        })?;
                    }
                }

                self.handle_events()?;
            }
            Ok(())
        }

        fn create_big_text() -> (BigText<'static>, Vec<Line<'static>>) {
            let text = BigTextBuilder::default()
                .pixel_size(tui_big_text::PixelSize::Quadrant)
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

        fn create_border(input: &str, show_popup: bool) -> Block {
            let title = if show_popup {
                "Hit Esc to close popup"
            } else {
                input
            };

            let border_style = if show_popup {
                Style::default().fg(ratatui::style::Color::Yellow)
            } else {
                Style::default().fg(ratatui::style::Color::White)
            };

            Block::new()
                .border_type(ratatui::widgets::BorderType::Rounded)
                .borders(Borders::TOP)
                .title(Line::from(title).centered())
                .style(border_style)
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
            let (tx, rx) = std::sync::mpsc::channel();
            Self {
                show_api_popup: false,
                show_api_dialog: ConfirmDialogState::default(),
                running: true,
                show_popup: false,
                selected_button: 1,
                popup_tx: tx,
                popup_rx: rx,
                popup_dialog: ConfirmDialogState::default(),
            }
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

            // Pass show_popup state to create_border
            let border = Home::create_border("Welcome", self.show_popup);
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
