use super::dashboard::Data;
use super::debug::DebugScreen;
use super::session::Device;
use super::{
    connection_progress::ConnectionProgress, help::help_popup::HelpPopup, host_type::HostTypePopup,
    popup::ApiPopup,
};
use crate::core_mod::widgets::TableWidgetItemManager;
use crate::events::input::{
    handle_backspace_key, handle_char_key, handle_d_key, handle_enter_key, handle_esc_key,
    handle_help_key, handle_left_key, handle_n_key, handle_q_key, handle_right_key,
};
use crate::screens::{
    error::error_widget::ErrorWidget, popup::InputBox, protocol_popup::ConnectionPopup,
};
use crate::state::{manager::manage_state, state::ScreenState};
use crate::{
    core_mod::{core::check_config, widgets::TableWidget},
    state::state::StateSnapshot,
};
use crossterm::event::{Event, KeyCode};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
    DefaultTerminal,
};
use std::sync::Arc;
use std::sync::Mutex;
use std::{error::Error, sync::mpsc};
use tui_big_text::{BigText, BigTextBuilder};
use tui_confirm_dialog::{ConfirmDialogState, Listener};

pub struct Home {
    pub running: bool,
    pub show_popup: bool,
    pub render_url_popup: bool,
    pub show_api_popup: bool,
    pub show_api_dialog: ConfirmDialogState,
    pub selected_button: usize,
    pub popup_tx: mpsc::Sender<Listener>,
    pub current_screen: ScreenState,
    pub popup_rx: mpsc::Receiver<Listener>,
    pub popup_dialog: ConfirmDialogState,
    pub error: bool,
}

impl Home {
    pub fn handle_event(
        &mut self,
        event: Event,
        input_box: Arc<Mutex<InputBox>>,
        table: Arc<Mutex<TableWidget>>,
        connection: Arc<Mutex<ConnectionPopup>>,
        error: Arc<Mutex<ErrorWidget>>,
        host: Arc<Mutex<HostTypePopup>>,
        debug_screen: Arc<Mutex<DebugScreen>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Event::Key(key) = event {
            match key.code {
                // Quit the app
                KeyCode::Char('q') => {
                    let mut input_box = input_box.lock().unwrap();
                    let mut connection = connection.lock().unwrap();
                    handle_q_key(self, &mut input_box, &mut connection);
                }

                // Start a new session
                KeyCode::Char('n') => {
                    let mut input_box = input_box.lock().unwrap();
                    let mut connection = connection.lock().unwrap();
                    handle_n_key(self, 'n', &mut input_box, &mut connection);
                }

                // Navigation keys for the table
                KeyCode::Down => {
                    let mut table = table.lock().unwrap();
                    table.next();
                }
                KeyCode::Up => {
                    let mut table = table.lock().unwrap();
                    table.previous();
                }

                // Close popups or escape actions
                KeyCode::Esc => {
                    let mut input_box = input_box.lock().unwrap();
                    handle_esc_key(self, &mut input_box);
                }

                // Handle left and right navigation
                KeyCode::Right => {
                    let mut input_box = input_box.lock().unwrap();
                    let mut connection = connection.lock().unwrap();
                    let mut host = host.lock().unwrap();
                    handle_right_key(self, &mut input_box, &mut connection, &mut host)
                }
                KeyCode::Left => {
                    let mut input_box = input_box.lock().unwrap();
                    let mut connection = connection.lock().unwrap();
                    let mut host = host.lock().unwrap();
                    handle_left_key(self, &mut input_box, &mut connection, &mut host)
                }

                // Handle Enter key actions based on current context
                KeyCode::Enter => {
                    let mut input_box = input_box.lock().unwrap();
                    let mut error = error.lock().unwrap();
                    let mut table = table.lock().unwrap();
                    let mut connection = connection.lock().unwrap();
                    let mut host = host.lock().unwrap();
                    handle_enter_key(
                        self,
                        &mut input_box,
                        &mut error,
                        &mut table,
                        &mut connection,
                        &mut host,
                    )
                }

                // Handle '?' key for help
                KeyCode::Char('?') => {
                    let mut table = table.lock().unwrap();
                    let mut input_box = input_box.lock().unwrap();
                    handle_help_key(self, &mut table, '?', &mut input_box);
                }

                // Handle regular character inputs
                KeyCode::Char(c) => {
                    if c == 'd' {
                        let mut debug_screen = debug_screen.lock().unwrap();
                        handle_d_key(self, &mut debug_screen);
                    }
                    let mut input_box = input_box.lock().unwrap();
                    handle_char_key(c, &mut input_box);
                }

                // Handle backspace key
                KeyCode::Backspace => {
                    let mut input_box = input_box.lock().unwrap();
                    handle_backspace_key(&mut input_box);
                }

                _ => {}
            }
        }

        Ok(())
    }

    pub async fn run(
        &mut self,
        term: Arc<Mutex<DefaultTerminal>>,
        event_rx: mpsc::Receiver<Event>,
    ) -> Result<(), Box<dyn Error>> {
        let input_box = Arc::new(Mutex::new(InputBox::default()));
        let progress = Arc::new(Mutex::new(ConnectionProgress::default()));
        let help = Arc::new(Mutex::new(HelpPopup::new()));
        let table = Arc::new(Mutex::new(TableWidget::new()));
        let connection = Arc::new(Mutex::new(ConnectionPopup::new()));
        let mut api_popup = ApiPopup::new();
        let error = Arc::new(Mutex::new(ErrorWidget::new()));
        let host = Arc::new(Mutex::new(HostTypePopup::new()));
        let mut session = Device::new_empty();
        let debug_screen = Arc::new(Mutex::new(DebugScreen::new()));
        session
            .add_item(
                Device {
                    files: Some(vec![
                        Data {
                            name: "File 1".to_string(),
                            status: Line::from(Span::styled(
                                "Not Sent",
                                Style::default().fg(ratatui::style::Color::Red),
                            )),
                            destination: "Urizen".to_string(),
                            time: "Just now".to_string(),
                        },
                        Data {
                            name: "File 2".to_string(),
                            status: Line::from(Span::styled(
                                "Sending",
                                Style::default().fg(ratatui::style::Color::Yellow),
                            )),
                            destination: "Urizen".to_string(),
                            time: "10 mins ago".to_string(),
                        },
                    ]),
                    name: "Urizen".to_string(),
                    last_connection: super::session::Connection {
                        total: "Just now".to_string(),
                        format_date: "Just now".to_string(),
                    },
                    last_transfer: super::session::Transfer {
                        status: "Not Sent".to_string(),
                        size: "Not Sent".to_string(),
                        name: "File 1".to_string(),
                    },
                    ip: "".to_string(),
                },
                table.clone(),
            )
            .await;
        while self.running {
            {
                if let Ok((selected_button, confirmed)) = self.popup_rx.try_recv() {
                    match confirmed {
                        Some(true) => {
                            self.show_api_popup = selected_button == 0;
                            self.render_url_popup = selected_button != 0;
                            self.show_popup = false;
                        }
                        Some(false) => {
                            self.show_popup = false;
                        }
                        None => {}
                    }
                }
            }

            match check_config() {
                Ok(_) => {
                    let state_snapshot = Arc::new(StateSnapshot {
                        table: table.clone(),
                        help: help.clone(),
                        connection: connection.clone(),
                        input_box: input_box.clone(),
                        host: host.clone(),
                        progress: progress.clone(),
                        debug_screen: debug_screen.clone(),
                    });

                    manage_state(self, state_snapshot, Arc::clone(&term)).unwrap();
                }
                Err(_) => {
                    let input_box_guard = input_box.lock();
                    let error_guard = error.lock();

                    term.lock().unwrap().draw(|f| {
                        let area = f.area();
                        self.render(area, f.buffer_mut());
                        if self.show_popup {
                            self.render_notification(f);
                        }
                        if self.show_api_popup {
                            api_popup.draw(f, &input_box_guard.unwrap());
                        }
                        if self.render_url_popup {
                            api_popup.render_url(f);
                        }
                        if self.error {
                            error_guard.unwrap().render_popup(f);
                        }
                    })?;
                }
            }

            let event = event_rx.recv().unwrap();
            self.handle_event(
                event,
                input_box.clone(),
                table.clone(),
                connection.clone(),
                error.clone(),
                host.clone(),
                debug_screen.clone(),
            )
            .unwrap();
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
        let commands = Paragraph::new(commands_text).alignment(ratatui::layout::Alignment::Right);
        commands.render(command_layout[1], buf);
    }

    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            current_screen: ScreenState::Sessions,
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

        let paragraph = Paragraph::new(normal_text).alignment(ratatui::layout::Alignment::Center);
        paragraph.render(content_layout[1], buf);

        Home::draw_commands(layout[2], buf);
    }
}
