use super::{
    connection_progress::ConnectionProgress, dashboard::Data, help::help_popup::HelpPopup,
    host_type::HostTypePopup, popup::ApiPopup, session::Device,
};
use crate::events::input::{
    handle_backspace_key, handle_char_key, handle_down_arrow, handle_enter_key, handle_esc_key,
    handle_help_key, handle_left_key, handle_n_key, handle_q_key, handle_right_key, handle_up_key,
};
use crate::screens::{
    error::error_widget::ErrorWidget, popup::InputBox, protocol_popup::ConnectionPopup,
};
use crate::state::{manager::manage_state, state::ScreenState};
use crate::utils::poll::poll_future;
use crate::utils::reset_state::StateReset;
use crate::{
    core_mod::{
        core::check_config,
        widgets::{TableWidget, TableWidgetItemManager},
    },
    state::state::StateSnapshot,
};
use core::panic;
use crossterm::event::{Event, KeyCode};
use futures::lock::Mutex;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::{
    error::Error,
    io,
    sync::{mpsc, Arc},
};
use tokio::sync::mpsc::Receiver;
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
    async fn handle_event(
        &mut self,
        event: Event,
        input_box: &mut InputBox,
        table: &mut TableWidget,
        connection: &mut ConnectionPopup,
        error: &mut ErrorWidget,
        host: &mut HostTypePopup,
        reset: &mut StateReset,
    ) -> io::Result<()> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('q') => handle_q_key(self, input_box, connection),
                KeyCode::Char('n') => handle_n_key(self, 'n', input_box, connection),
                KeyCode::Down => handle_down_arrow(self, table),
                KeyCode::Up => handle_up_key(self, table),
                KeyCode::Esc => handle_esc_key(self, input_box),
                KeyCode::Right => handle_right_key(self, input_box, connection, host),
                KeyCode::Left => handle_left_key(self, input_box, connection, host),
                KeyCode::Enter => {
                    handle_enter_key(self, input_box, error, table, connection, host, reset)
                }
                KeyCode::Char('?') => handle_help_key(self, table, '?', input_box),
                KeyCode::Char(c) => handle_char_key(self, c, input_box),
                KeyCode::Backspace => handle_backspace_key(self, input_box),
                _ => {}
            }
        }
        Ok(())
    }

    pub async fn run(
        &mut self,
        term: Arc<Mutex<DefaultTerminal>>,
        mut event_rx: Receiver<Event>,
    ) -> Result<(), Box<dyn Error>> {
        let mut input_box = InputBox::default();
        let mut progress = Arc::new(Mutex::new(ConnectionProgress::default()));
        let mut help = HelpPopup::new();
        let mut table = TableWidget::new();
        let mut connection = ConnectionPopup::new();
        let mut api_popup = ApiPopup::new();
        let mut error = ErrorWidget::new();
        let mut host = HostTypePopup::new();
        let mut state_handler_reset = StateReset::default();

        while self.running {
            // Handle popup events in a separate scope to limit borrow duration
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
                    term.lock().await.draw(|f| {
                        let state_snapshot = StateSnapshot {
                            home: self,
                            table: &mut table,
                            help: &mut help,
                            connection: &mut connection,
                            input_box: &mut input_box,
                            host: &mut host,
                            progress: &mut progress,
                        };

                        poll_future(Box::pin(manage_state(state_snapshot, f)))
                    })?;
                }
                Err(_) => {
                    term.lock().await.draw(|f| {
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

            if let Some(event) = event_rx.recv().await {
                self.handle_event(
                    event,
                    &mut input_box,
                    &mut table,
                    &mut connection,
                    &mut error,
                    &mut host,
                    &mut state_handler_reset,
                )
                .await?;
            }
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
        let (tx, rx) = std::sync::mpsc::channel();
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

        let paragraph = Paragraph::new(normal_text).alignment(ratatui::layout::Alignment::Center);
        paragraph.render(content_layout[1], buf);

        Home::draw_commands(layout[2], buf);
    }
}
