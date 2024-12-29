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
    use std::sync::{Arc, Mutex};
    use std::{error::Error, io};
    use tui_big_text::{BigText, BigTextBuilder};
    use tui_confirm_dialog::{ConfirmDialogState, Listener};

    use crate::core::core_lib::check_config;
    use crate::error::error_widget::ErrorWidget;
    use crate::help::help_popup::HelpPopup;
    use crate::input::{
        handle_backspace_key, handle_char_key, handle_down_arrow, handle_enter_key, handle_esc_key,
        handle_help_key, handle_left_key, handle_n_key, handle_q_key, handle_right_key,
        handle_up_key,
    };
    use crate::popup::{ApiPopup, InputBox};
    use crate::protocol::protocol_popup::ConnectionPopup;
    use crate::sessions::{draw_session_table_ui, Device};
    use crate::widget::{TableWidget, TableWidgetItemManager};

    pub struct Home {
        pub running: bool,
        pub show_popup: bool,
        pub render_url_popup: bool,
        pub show_api_popup: bool,
        pub show_api_dialog: ConfirmDialogState,
        pub selected_button: usize,
        pub popup_tx: mpsc::Sender<Listener>,
        pub popup_rx: mpsc::Receiver<Listener>,
        pub popup_dialog: ConfirmDialogState,
        pub error: bool,
    }

    impl Home {
        pub fn handle_events(
            &mut self,
            input_box: &mut InputBox,
            table: &mut TableWidget,
            connection: &mut ConnectionPopup,
            error: &mut ErrorWidget,
        ) -> io::Result<()> {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => handle_q_key(self, input_box, table),
                    KeyCode::Char('n') => handle_n_key(self, 'n', input_box, table),
                    KeyCode::Down => handle_down_arrow(self, table),
                    KeyCode::Up => handle_up_key(self, table),
                    KeyCode::Esc => handle_esc_key(self, input_box),
                    KeyCode::Right => {
                        if table.connection {
                            connection.selected = connection.selected.next_val();
                            return Ok(());
                        }
                        handle_right_key(self, input_box)
                    }
                    KeyCode::Left => {
                        if table.connection {
                            connection.selected = connection.selected.previous_val();
                            return Ok(());
                        }
                        handle_left_key(self, input_box)
                    }
                    KeyCode::Enter => {
                        if table.connection {
                            connection.return_selected(table);
                            return Ok(());
                        }
                        handle_enter_key(self, input_box, error)
                    }
                    KeyCode::Char('?') => handle_help_key(self, table, '?', input_box),
                    KeyCode::Char(c) => handle_char_key(self, c, input_box),
                    KeyCode::Backspace => handle_backspace_key(self, input_box),
                    _ => {}
                }
            }
            Ok(())
        }

        pub fn run(mut self, term: Arc<Mutex<DefaultTerminal>>) -> Result<(), Box<dyn Error>> {
            let mut input_box = InputBox::default();
            let mut help = HelpPopup::new();
            let mut table = TableWidget::new();
            let mut session_table = Device::new_empty();
            session_table.add_item(
                Device {
                    name: "Urizen".to_string(),
                    last_connection: crate::sessions::Connection {
                        total: "Just now".to_string(),
                        format_date: "Just now".to_string(),
                    },
                    last_transfer: crate::sessions::Transfer {
                        status: "Not Sent".to_string(),
                        size: "Not Sent".to_string(),
                        name: "File 1".to_string(),
                    },
                    ip: "".to_string(),
                },
                &mut table,
            );
            session_table.add_item(
                Device {
                    name: "Urizen".to_string(),
                    last_connection: crate::sessions::Connection {
                        total: "Just now".to_string(),
                        format_date: "Just now".to_string(),
                    },
                    last_transfer: crate::sessions::Transfer {
                        status: "Not Sent".to_string(),
                        size: "Not Sent".to_string(),
                        name: "File 1".to_string(),
                    },
                    ip: "".to_string(),
                },
                &mut table,
            );
            // let status = Line::from(Span::styled(
            //     "Not Sent",
            //     Style::default().fg(ratatui::style::Color::Red),
            // ));
            // table.add_item(
            //     "File 1".to_string(),
            //     status,
            //     "Urizen".to_string(),
            //     "Just now".to_string(),
            // );
            // table.add_item(
            //     "File 2".to_string(),
            //     Line::from(Span::styled(
            //         "Sending",
            //         Style::default().fg(ratatui::style::Color::Yellow),
            //     )),
            //     "Urizen".to_string(),
            //     "10 mins ago".to_string(),
            // );
            // table.add_item(
            //     "File 1".to_string(),
            //     Line::from(Span::styled(
            //         "Sent",
            //         Style::default().fg(ratatui::style::Color::Green),
            //     )),
            //     "Urizen".to_string(),
            //     "Just now".to_string(),
            // );
            let mut connection = ConnectionPopup::new();
            let mut api_popup = ApiPopup::new();
            let mut error = ErrorWidget::new();
            while self.running {
                if let Ok((selected_button, confirmed)) = self.popup_rx.try_recv() {
                    match confirmed {
                        Some(true) => {
                            if selected_button == 0 {
                                self.show_api_popup = true;
                            } else {
                                self.render_url_popup = true;
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
                        term.lock().unwrap().draw(|f| {
                            draw_session_table_ui(f, &mut table);
                            if table.help {
                                help.draw_dashboard_help(f);
                            }
                            if table.connection {
                                connection.render(f);
                            }
                            if connection.input_popup {
                                connection.draw_input(f);
                            }
                        })?;
                    }
                    Err(_) => {
                        term.lock().unwrap().draw(|f| {
                            let area = f.area();
                            self.render(area, f.buffer_mut());
                            if self.show_popup {
                                self.render_notification(f);
                            }
                            if self.show_api_popup {
                                api_popup.draw(f, &input_box);
                            }
                            if self.render_url_popup {
                                api_popup.render_url(f);
                            }
                            if self.error {
                                error.render_popup(f);
                            }
                        })?;
                    }
                }

                self.handle_events(&mut input_box, &mut table, &mut connection, &mut error)?;
            }
            Ok(())
        }

        fn create_big_text() -> (BigText<'static>, Vec<Line<'static>>) {
            let text = BigTextBuilder::default()
                .pixel_size(tui_big_text::PixelSize::Quadrant)
                .lines(["ZYNC".into()])
                .style(Style::default().fg(ratatui::style::Color::Red))
                .build();
            let line = Home::create_line();
            (text, line)
        }

        fn create_line() -> Vec<Line<'static>> {
            let line = "Welcome to Zync. Hit n to start your new file sharing session.";
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
                error: false,
                show_api_popup: false,
                show_api_dialog: ConfirmDialogState::default(),
                running: true,
                show_popup: false,
                render_url_popup: false,
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
