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

    use crate::core::core_lib::{check_config, create_config};
    use crate::error::error_widget::ErrorWidget;
    use crate::help::help_popup::HelpPopup;
    use crate::popup::{ApiPopup, InputBox, InputMode, FLAG};
    use crate::protocol::protocol_popup::ConnectionPopup;
    use crate::sessions::{draw_session_table_ui, Device};
    use crate::widget::{TableWidget, TableWidgetItemManager};

    pub struct Home {
        running: bool,
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
                    KeyCode::Char('q') => self.handle_q_key(input_box, table),
                    KeyCode::Char('n') => self.handle_n_key('n', input_box, table),
                    KeyCode::Down => self.handle_down_arrow(table),
                    KeyCode::Up => self.handle_up_key(table),
                    KeyCode::Esc => self.handle_esc_key(input_box),
                    KeyCode::Right => {
                        if table.connection {
                            connection.selected = connection.selected.next_val();
                            return Ok(());
                        }
                        self.handle_right_key(input_box)
                    }
                    KeyCode::Left => {
                        if table.connection {
                            connection.selected = connection.selected.previous_val();
                            return Ok(());
                        }
                        self.handle_left_key(input_box)
                    }
                    KeyCode::Enter => {
                        if table.connection {
                            connection.return_selected(table);
                            return Ok(());
                        }
                        self.handle_enter_key(input_box, error)
                    }
                    KeyCode::Char('?') => self.handle_help_key(table, '?', input_box),
                    KeyCode::Char(c) => self.handle_char_key(c, input_box),
                    KeyCode::Backspace => self.handle_backspace_key(input_box),
                    _ => {}
                }
            }
            Ok(())
        }
        fn handle_help_key(
            &mut self,
            table: &mut TableWidget,
            key: char,
            input_box: &mut InputBox,
        ) {
            if !self.show_api_popup {
                table.help = !table.help;
            }
            self.handle_char_key(key, input_box);
        }

        fn handle_q_key(&mut self, input_box: &mut InputBox, table: &mut TableWidget) {
            match (
                self.show_api_popup,
                self.show_popup,
                self.render_url_popup,
                table.help,
                table.connection,
            ) {
                (true, _, _, _, _) => self.handle_char_key('q', input_box),
                (_, true, _, _, _) => {
                    self.popup_tx
                        .send((self.selected_button as u16, Some(false)))
                        .unwrap();
                    self.show_popup = false;
                }
                (_, _, true, _, _) => {}
                (_, _, _, true, _) => {}
                (_, _, _, _, true) => {}
                _ => self.running = false,
            }
        }

        fn handle_n_key(&mut self, c: char, input_box: &mut InputBox, table: &mut TableWidget) {
            if self.show_api_popup {
                self.handle_char_key(c, input_box);
            } else if !self.show_popup {
                self.show_popup = true;
            } else {
                table.connection = !table.connection;
            }
        }

        fn handle_esc_key(&mut self, input_box: &mut InputBox) {
            if self.show_popup {
                self.popup_tx
                    .send((self.selected_button as u16, Some(false)))
                    .unwrap();
                self.show_popup = false;
            } else if input_box.input_mode == InputMode::Editing {
                input_box.input_mode = InputMode::Normal;
                unsafe { FLAG = false };
            } else if self.show_api_popup {
                self.show_api_popup = false;
            } else if self.render_url_popup {
                self.render_url_popup = false;
            } else if self.error {
                self.error = false;
            }
        }

        fn handle_right_key(&mut self, input_box: &mut InputBox) {
            if self.show_popup {
                self.selected_button = (self.selected_button + 1) % 2;
                self.popup_tx
                    .send((self.selected_button as u16, None))
                    .unwrap();
            } else if input_box.input_mode == InputMode::Editing {
                input_box.move_cursor_right();
            }
        }

        fn handle_left_key(&mut self, input_box: &mut InputBox) {
            if self.show_popup {
                self.selected_button = (self.selected_button + 1) % 2;
                self.popup_tx
                    .send((self.selected_button as u16, None))
                    .unwrap();
            } else if input_box.input_mode == InputMode::Editing {
                input_box.move_cursor_left();
            }
        }
        fn handle_enter_key(&mut self, input_box: &mut InputBox, error: &mut ErrorWidget) {
            if self.show_popup {
                self.popup_tx
                    .send((self.selected_button as u16, Some(true)))
                    .unwrap();
                self.show_popup = false;
            } else {
                match input_box.input_mode == InputMode::Editing {
                    true => {
                        let api = input_box.submit_message();
                        match api {
                            Ok(api) => {
                                create_config(&api).unwrap();
                                self.show_api_popup = false;
                            }
                            Err(err) => {
                                self.show_api_popup = false;
                                error.set_val(
                                    err.to_string(),
                                    &mut crate::error::error_widget::ErrorType::Warning,
                                    "Ok".to_string(),
                                );
                                self.error = true;
                            }
                        }
                    }
                    false => {
                        let output = input_box.submit_message();
                        match output {
                            Ok(key) => {
                                create_config(&key).unwrap();
                                self.show_api_popup = false;
                            }
                            Err(err) => {
                                self.show_api_popup = false;
                                error.set_val(
                                    err.to_string(),
                                    &mut crate::error::error_widget::ErrorType::Warning,
                                    "Ok".to_string(),
                                );
                                self.error = true;
                            }
                        }
                    }
                }
            }
        }

        fn handle_char_key(&mut self, c: char, input_box: &mut InputBox) {
            if input_box.input_mode == InputMode::Editing {
                input_box.enter_char(c);
            } else if c == 'e' {
                input_box.input_mode = InputMode::Editing;
                unsafe { FLAG = true };
            }
        }
        fn handle_up_key(&mut self, table: &mut TableWidget) {
            if !table.help && !table.connection {
                table.previous();
            }
        }
        fn handle_down_arrow(&mut self, table: &mut TableWidget) {
            if !table.help && !table.connection {
                table.next();
            }
        }

        fn handle_backspace_key(&mut self, input_box: &mut InputBox) {
            if input_box.input_mode == InputMode::Editing {
                input_box.delete_char();
            }
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
