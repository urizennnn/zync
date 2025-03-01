use derive_setters::Setters;
use ratatui::layout::{Constraint, Layout, Position};
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::text::{Span, Text};
use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::{Frame, layout::Alignment, style::Color};
use ratatui::{layout::Rect, text::Line, widgets::Clear};
use tui_confirm_dialog::{ButtonLabel, ConfirmDialog, ConfirmDialogState};

use crate::utils::calculate::calculate_popup_area;

use super::home::Home;

pub static mut FLAG: bool = false;

#[derive(Setters)]
pub struct ApiPopup {
    pub title: String,
    pub message: String,
    pub input: InputBox,
    pub error: bool,
}

impl ApiPopup {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            error: false,
            title: "API Key".to_string(),
            message: "Please input your API key".to_string(),
            input: InputBox::new(),
        }
    }

    pub fn render_url(&mut self, frame: &mut Frame) {
        let spans = vec![
            Span::raw("Copy "),
            Span::styled("this url", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(
                " and paste it in your browser of choice to acquire your API key \
                and proceed with your file sharing session.\n",
            ),
            Span::styled(
                "http://localhost:8000",
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ];

        let text = Text::from(Line::from(spans));

        let url = Paragraph::new(text)
            .block(
                Block::default()
                    .borders(ratatui::widgets::Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Gray))
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);

        let area = calculate_popup_area(frame.area(), 30, 10);
        frame.render_widget(Clear, area);
        frame.render_widget(url, area);
    }

    pub fn draw(&self, frame: &mut Frame, input: &InputBox) {
        let area = calculate_popup_area(frame.area(), 30, 30);
        frame.render_widget(Clear, area);

        let popup_layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(area);

        let title = Paragraph::new(self.title.clone())
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Yellow));
        frame.render_widget(title, popup_layout[0]);

        let message = Paragraph::new(self.message.clone()).alignment(Alignment::Center);
        frame.render_widget(message, popup_layout[2]);

        let input_area = popup_layout[3];
        let input_layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(input_area);

        let padded_input_area = Layout::horizontal([
            Constraint::Length(2),
            Constraint::Min(1),
            Constraint::Length(2),
        ])
        .split(input_layout[1])[1];

        input.draw_in_popup(frame, padded_input_area);

        let help_area = popup_layout[4];
        input.draw_help(frame, help_area);

        let popup_block = Block::default()
            .borders(ratatui::widgets::Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .style(Style::default().fg(Color::Gray));
        frame.render_widget(popup_block, area);
    }
}

#[derive(Debug)]
pub struct InputBox {
    pub input: String,
    pub character_index: usize,
    pub input_mode: InputMode,
    pub removed_char: Vec<char>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum InputMode {
    Normal,
    Editing,
}

impl InputBox {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            input_mode: InputMode::Normal,
            character_index: 0,
            input: String::new(),
            removed_char: vec![],
        }
    }
    fn return_cloned_char(&mut self) -> Vec<char> {
        let char_vec = self.removed_char.clone();
        let mut new_vec: Vec<char> = vec![];
        for char in char_vec.iter() {
            new_vec.push(*char);
        }
        new_vec
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);

        if self.character_index == 0 {
            let chars = self.return_cloned_char();
            let mut flag = true; // use flag as a toggle control

            for (i, &char) in chars.iter().rev().enumerate() {
                if i == chars.len() - 1 {
                    // toggle `flag` when last char is reached
                    flag = false;
                }
                if flag {
                    self.input.insert(0, char);
                }
            }
        }
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        if self.character_index as u16 == 65 {
            let char = self.input.remove(0);
            self.removed_char.push(char);
        }
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    pub fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    pub fn submit_message(&mut self) -> Result<String, &'static str> {
        let mut input_msg = self.input.clone();

        if input_msg.is_empty() {
            return Err("API key cannot be empty");
        }

        self.input.clear();
        self.reset_cursor();

        for char in self.removed_char.iter().rev() {
            input_msg.insert(0, *char);
        }

        Ok(input_msg)
    }

    pub fn draw_in_popup(&self, frame: &mut Frame, mut area: Rect) {
        let input_mode = if unsafe { FLAG } {
            InputMode::Editing
        } else {
            InputMode::Normal
        };
        area.height += 1;

        let input = Paragraph::new(self.input.as_str())
            .style(match input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(
                Block::default()
                    .borders(ratatui::widgets::Borders::ALL)
                    .title("Input"),
            );
        frame.render_widget(input, area);

        if input_mode == InputMode::Editing {
            frame.set_cursor_position(Position::new(
                area.x + self.character_index as u16 + 1,
                area.y + 1,
            ));
        }
    }

    pub fn draw_help(&self, frame: &mut Frame, area: Rect) {
        let input_mode = if unsafe { FLAG } {
            InputMode::Editing
        } else {
            InputMode::Normal
        };

        let (msg, style) = match input_mode {
            InputMode::Normal => (
                vec![
                    "Press ".into(),
                    "e".bold(),
                    " to start editing.".into(),
                    " Press ".into(),
                    "Esc".bold(),
                    " to quit.".into(),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            InputMode::Editing => (
                vec![
                    "Press ".into(),
                    "Esc".bold(),
                    " to stop editing, ".into(),
                    "Enter".bold(),
                    " to submit".into(),
                ],
                Style::default(),
            ),
        };
        let text = Text::from(Line::from(msg)).patch_style(style);
        let help_message = Paragraph::new(text).alignment(Alignment::Center);
        frame.render_widget(help_message, area);
    }
}

impl Home {
    pub fn render_notification(&mut self, frame: &mut Frame) {
        self.popup_dialog = ConfirmDialogState::default()
            .modal(true)
            .with_title(Line::from("Notification").cyan().centered())
            .with_text(vec![Line::from("Do you have an api key?")])
            .with_yes_button(ButtonLabel::from("(Y)es").unwrap())
            .with_no_button(ButtonLabel::from("(N)o").unwrap())
            .with_yes_button_selected(self.selected_button == 0)
            .with_listener(Some(self.popup_tx.clone()))
            .open();

        let area = calculate_popup_area(frame.area(), 50, 30);

        if self.popup_dialog.is_opened() {
            let popup = ConfirmDialog::default()
                .borders(ratatui::widgets::Borders::ALL)
                .bg(ratatui::style::Color::Black)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .button_style(ratatui::prelude::Style::default())
                .selected_button_style(
                    ratatui::prelude::Style::default()
                        .fg(ratatui::style::Color::Yellow)
                        .bold(),
                );

            frame.render_widget(Clear, area);
            frame.render_stateful_widget(popup, area, &mut self.popup_dialog);
        }
    }
}
