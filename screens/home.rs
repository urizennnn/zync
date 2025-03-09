use super::debug::DebugScreen;
use super::host_type::HostTypePopup;
use super::session::{Connection, Device, Transfer};
use crate::core_mod::widgets::{Item, TableWidget};
use crate::core_mod::{self};
use crate::events::input::{
    handle_backspace_key, handle_char_key, handle_d_key, handle_enter_key, handle_esc_key,
    handle_help_key, handle_left_key, handle_n_key, handle_o_key, handle_q_key, handle_right_key,
};
use crate::events::ui_update::UIUpdate;
use crate::init::GLOBAL_RUNTIME;
use crate::internal::handle_upload::handle_incoming_upload;
use crate::internal::session_store::load_sessions;
use crate::screens::{
    error::error_widget::ErrorWidget, popup::InputBox, protocol_popup::ConnectionPopup,
};
use crate::state::state::ScreenState;
use crossterm::event::{Event, KeyCode};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::text::Span;
use ratatui::widgets::Borders;
use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tui_big_text::BigText;

pub struct HomeDeps {
    pub input_box: Arc<Mutex<InputBox>>,
    pub table: Arc<Mutex<TableWidget>>,
    pub connection: Arc<Mutex<ConnectionPopup>>,
    pub error: Arc<Mutex<ErrorWidget>>,
    pub host: Arc<Mutex<HostTypePopup>>,
    pub debug_screen: Arc<Mutex<DebugScreen>>,
    pub progress: Arc<Mutex<crate::screens::connection_progress::ConnectionProgress>>,
    pub state_snapshot: Arc<crate::state::state::StateSnapshot>,
}

pub struct Home {
    pub running: bool,
    pub show_popup: bool,
    pub render_url_popup: bool,
    pub show_api_popup: bool,
    pub show_api_dialog: tui_confirm_dialog::ConfirmDialogState,
    pub selected_button: usize,
    pub popup_tx: Sender<tui_confirm_dialog::Listener>,
    pub popup_rx: Receiver<tui_confirm_dialog::Listener>,
    pub popup_dialog: tui_confirm_dialog::ConfirmDialogState,
    pub error: bool,
    pub ui_update_tx: Sender<UIUpdate>,
    pub ui_update_rx: Receiver<UIUpdate>,
    pub popup_message: Option<String>,
    pub current_screen: ScreenState,
    pub tcp_stream: Option<Arc<Mutex<tokio::net::TcpStream>>>,
}

impl Home {
    pub fn handle_event(
        &mut self,
        event: Event,
        deps: &mut HomeDeps,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('q') => {
                    let mut input_box = deps.input_box.lock().unwrap();
                    let mut connection = deps.connection.lock().unwrap();
                    handle_q_key(self, &mut input_box, &mut connection);
                }
                KeyCode::Char('n') => {
                    let mut input_box = deps.input_box.lock().unwrap();
                    let mut connection = deps.connection.lock().unwrap();
                    handle_n_key(self, 'n', &mut input_box, &mut connection);
                }
                KeyCode::Down => {
                    let mut table = deps.table.lock().unwrap();
                    table.next();
                }
                KeyCode::Up => {
                    let mut table = deps.table.lock().unwrap();
                    table.previous();
                }
                KeyCode::Esc => {
                    let mut input_box = deps.input_box.lock().unwrap();
                    handle_esc_key(self, &mut input_box);
                }
                KeyCode::Char('o') => {
                    let mut debug_screen = deps.debug_screen.lock().unwrap();
                    handle_o_key(self, &deps.state_snapshot, &mut debug_screen);
                }
                KeyCode::Right => {
                    let mut input_box = deps.input_box.lock().unwrap();
                    let mut connection = deps.connection.lock().unwrap();
                    let mut host = deps.host.lock().unwrap();
                    handle_right_key(self, &mut input_box, &mut connection, &mut host);
                }
                KeyCode::Left => {
                    let mut input_box = deps.input_box.lock().unwrap();
                    let mut connection = deps.connection.lock().unwrap();
                    let mut host = deps.host.lock().unwrap();
                    handle_left_key(self, &mut input_box, &mut connection, &mut host);
                }
                KeyCode::Enter => {
                    let mut input_box = deps.input_box.lock().unwrap();
                    let mut error = deps.error.lock().unwrap();
                    let mut table = deps.table.lock().unwrap();
                    let mut host = deps.host.lock().unwrap();
                    let progress = deps.progress.clone();

                    handle_enter_key(
                        self,
                        &mut input_box,
                        &mut error,
                        &mut table,
                        deps.connection.clone(),
                        &mut host,
                        progress,
                    );
                }
                KeyCode::Char('?') => {
                    let mut table = deps.table.lock().unwrap();
                    let mut input_box = deps.input_box.lock().unwrap();
                    handle_help_key(self, &mut table, '?', &mut input_box);
                }
                KeyCode::Char(c) => {
                    if c == 'd' {
                        let mut debug_screen = deps.debug_screen.lock().unwrap();
                        handle_d_key(self, &mut debug_screen);
                    }
                    let mut input_box = deps.input_box.lock().unwrap();
                    handle_char_key(c, &mut input_box);
                }
                KeyCode::Backspace => {
                    let mut input_box = deps.input_box.lock().unwrap();
                    handle_backspace_key(&mut input_box);
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn run(
        &mut self,
        term: Arc<Mutex<ratatui::DefaultTerminal>>,
        event_rx: Receiver<Event>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let input_box = Arc::new(Mutex::new(crate::screens::popup::InputBox::new()));
        let progress = Arc::new(Mutex::new(
            crate::screens::connection_progress::ConnectionProgress::default(),
        ));
        let help = Arc::new(Mutex::new(
            crate::screens::help::help_popup::HelpPopup::new(),
        ));
        let table = Arc::new(Mutex::new(TableWidget::new()));
        let connection = Arc::new(Mutex::new(
            crate::screens::protocol_popup::ConnectionPopup::new(),
        ));
        let mut api_popup = crate::screens::popup::ApiPopup::new();
        let error = Arc::new(Mutex::new(ErrorWidget::new()));
        let host = Arc::new(Mutex::new(HostTypePopup::new()));
        let debug_screen = Arc::new(Mutex::new(DebugScreen::new()));

        {
            let records = load_sessions();
            let mut t = table.lock().unwrap();
            for rec in records {
                t.items.push(Item::Device(Device {
                    name: rec.name,
                    ip: rec.ip,
                    last_transfer: Transfer {
                        status: rec.last_transfer,
                        size: "N/A".to_string(),
                        name: "N/A".to_string(),
                    },
                    last_connection: Connection {
                        total: rec.last_connection.clone(),
                        format_date: rec.last_connection,
                    },
                    files: None,
                }));
            }
        }

        while self.running {
            while let Ok(update) = self.ui_update_rx.try_recv() {
                match update {
                    UIUpdate::ShowPopup(msg) => {
                        self.popup_message = Some(msg);
                        self.show_popup = true;
                    }
                    UIUpdate::SwitchScreen(screen) => {
                        self.current_screen = screen;
                    }
                }
            }

            let state_snapshot = Arc::new(crate::state::state::StateSnapshot {
                table: table.clone(),
                help: help.clone(),
                connection: connection.clone(),
                input_box: input_box.clone(),
                host: host.clone(),
                progress: progress.clone(),
                debug_screen: debug_screen.clone(),
                stream: self.tcp_stream.clone(),
            });

            let mut deps = HomeDeps {
                input_box: input_box.clone(),
                table: table.clone(),
                connection: connection.clone(),
                error: error.clone(),
                host: host.clone(),
                debug_screen: debug_screen.clone(),
                progress: progress.clone(),
                state_snapshot: state_snapshot.clone(),
            };

            match core_mod::core::check_config() {
                Ok(_) => {
                    crate::state::manager::manage_state(
                        self,
                        state_snapshot.clone(),
                        term.clone(),
                    )?;
                }
                Err(_) => {
                    let input_box_guard = input_box.lock().unwrap();
                    let error_guard = error.lock().unwrap();
                    term.lock().unwrap().draw(|f| {
                        let area = f.area();
                        self.render(area, f.buffer_mut());
                        if self.show_popup {
                            self.render_notification(f);
                        }
                        if self.show_api_popup {
                            api_popup.draw(f, &input_box_guard);
                        }
                        if self.render_url_popup {
                            api_popup.render_url(f);
                        }
                        if self.error {
                            error_guard.render_centered_popup(f);
                        }
                    })?;
                }
            }

            if let Ok(event) = event_rx.recv_timeout(Duration::from_millis(100)) {
                self.handle_event(event, &mut deps)?;
            }
        }
        Ok(())
    }

    fn create_big_text() -> (BigText<'static>, Vec<Line<'static>>) {
        let text = tui_big_text::BigTextBuilder::default()
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
    pub fn spawn_upload_handler_if_needed(&mut self) {
        static UPLOAD_TASK_SPAWNED: once_cell::sync::OnceCell<bool> =
            once_cell::sync::OnceCell::new();

        if self.tcp_stream.is_none() {
            return;
        }
        if UPLOAD_TASK_SPAWNED.get().is_some() {
            return;
        }
        UPLOAD_TASK_SPAWNED.set(true).ok();

        let stream_arc = Arc::clone(self.tcp_stream.as_ref().unwrap());
        GLOBAL_RUNTIME.spawn(async move {
            let buffer = vec![0u8; 5_242_880];
            let result = tokio::task::spawn_blocking({
                let stream_arc = Arc::clone(&stream_arc);
                let mut buffer_clone = buffer.clone();
                move || {
                    let mut guard = stream_arc.lock().unwrap();
                    GLOBAL_RUNTIME.block_on(handle_incoming_upload(&mut guard, &mut buffer_clone))
                }
            })
            .await;

            match result {
                Ok(Ok(())) => {}
                Ok(Err(_e)) => {
                    // Removed the eprintln! calls here
                }
                Err(_e) => {
                    // Removed the eprintln! call here
                }
            }
            tokio::task::yield_now().await;
        });
    }

    pub fn new() -> Self {
        let (tx, rx) = channel();
        let (ui_tx, ui_rx) = channel();
        Self {
            current_screen: ScreenState::Sessions,
            error: false,
            show_api_popup: false,
            show_api_dialog: tui_confirm_dialog::ConfirmDialogState::default(),
            running: true,
            show_popup: false,
            render_url_popup: false,
            selected_button: 1,
            popup_tx: tx,
            popup_rx: rx,
            popup_dialog: tui_confirm_dialog::ConfirmDialogState::default(),
            ui_update_tx: ui_tx,
            ui_update_rx: ui_rx,
            popup_message: None,
            tcp_stream: None,
        }
    }
}

impl Default for Home {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &Home {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        use ratatui::layout::{Constraint, Direction, Layout};
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Min(10),
                Constraint::Length(1),
            ])
            .split(area);

        let border = Home::create_border("Welcome", self.show_popup);
        border.render(layout[0], buf);

        let (big_text, normal_text) = Home::create_big_text();
        let content_layout = Layout::default()
            .direction(Direction::Vertical)
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
