use crate::core_mod::widgets::Item;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};
use std::sync::{Arc, Mutex};
use unicode_width::UnicodeWidthStr;

use crate::core_mod::widgets::{TableWidget, TableWidgetItemManager};

use super::dashboard::Data;

#[derive(Debug, Clone)]
pub struct Device {
    pub name: String,
    pub ip: String,
    pub last_transfer: Transfer,
    pub last_connection: Connection,
    pub files: Option<Vec<Data>>,
}

impl Device {
    pub fn new_empty() -> Self {
        Self {
            files: None,
            name: String::new(),
            ip: String::new(),
            last_transfer: Transfer {
                status: String::new(),
                size: String::new(),
                name: String::new(),
            },
            last_connection: Connection {
                total: String::new(),
                format_date: String::new(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Transfer {
    pub status: String,
    pub size: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Connection {
    pub total: String,
    pub format_date: String,
}

impl TableWidgetItemManager for Device {
    type Item = Device;

    async fn add_item(&mut self, item: Self::Item, table: Arc<Mutex<TableWidget>>) {
        // Unwrap the mutex lock to access the inner TableWidget.
        let mut table_guard = table.lock().expect("Mutex poisoned in add_item");
        table_guard.items.push(Item::Device(item));
        let device_items: Vec<&Self::Item> = table_guard
            .items
            .iter()
            .filter_map(|item| {
                if let Item::Device(device) = item {
                    Some(device)
                } else {
                    None
                }
            })
            .collect();

        table_guard.longest_item_lens = Self::constraint_len_calculator(&device_items);
    }
    fn constraint_len_calculator(items: &[&Self::Item]) -> Vec<u16> {
        let width: u16 = items
            .iter()
            .map(|item| {
                item.name.width() as u16
                    + item.last_connection.format_date.width() as u16
                    + item.last_transfer.status.width() as u16
            })
            .max()
            .unwrap_or(0);

        let name_percent = items
            .iter()
            .map(|item| item.name.width() as u16)
            .max()
            .unwrap_or(0)
            * 100
            / width;

        let date_percent = items
            .iter()
            .map(|item| item.last_connection.format_date.width() as u16)
            .max()
            .unwrap_or(0)
            * 100
            / width;

        let status_percent = items
            .iter()
            .map(|item| item.last_transfer.status.width() as u16)
            .max()
            .unwrap_or(0)
            * 100
            / width;

        vec![name_percent, date_percent, status_percent]
    }
}

pub fn session_details_ui(table: &mut TableWidget) -> Paragraph<'static> {
    let item_index = table.state.selected().unwrap_or(0);
    if let Some(Item::Device(device)) = table.items.get(item_index) {
        let details = vec![
            Line::from(vec!["Name: ".into(), device.name.clone().yellow()]),
            Line::from(vec!["IP Address: ".into(), device.ip.clone().green()]),
            Line::from(""),
            Line::from(vec![
                "Last Transfer: ".into(),
                device.last_transfer.name.clone().blue(),
                " (".into(),
                device.last_transfer.size.clone().white(),
                ")".into(),
            ]),
            Line::from(vec![
                "Status: ".into(),
                device.last_transfer.status.clone().cyan(),
            ]),
            Line::from(""),
            Line::from(vec!["Connection History:".bold()]),
            Line::from(vec![
                "  Total: ".into(),
                device.last_connection.total.clone().white(),
            ]),
            Line::from(vec![
                "  First: ".into(),
                device.last_connection.format_date.clone().white(),
            ]),
        ];

        return Paragraph::new(Text::from(details))
            .block(Block::default().borders(Borders::ALL).title("Details"))
            .alignment(Alignment::Left);
    }

    Paragraph::new(Line::from("No details available"))
        .block(Block::default().borders(Borders::ALL).title("Details"))
        .alignment(Alignment::Center)
}

pub fn session_table_ui(table: &mut TableWidget) -> ratatui::widgets::Table<'_> {
    let header_style = Style::default()
        .fg(table.colors.header_fg)
        .bg(table.colors.header_bg);
    let selected_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(table.colors.selected_style_fg);
    let header = ["Computer", "Last Connected", "Status"]
        .iter()
        .map(|&s| Cell::from(s))
        .collect::<Row>()
        .style(header_style)
        .height(1);

    let rows = table.items.iter().enumerate().filter_map(|(i, data)| {
        if let Item::Device(device) = data {
            let color = match i % 2 {
                0 => table.colors.normal_row_color,
                _ => table.colors.alt_row_color,
            };

            let cells = vec![
                Cell::from(device.name.clone()),
                Cell::from(device.last_connection.format_date.clone()),
                Cell::from(device.last_transfer.status.clone()),
            ];

            Some(
                Row::new(cells)
                    .style(Style::new().fg(table.colors.row_fg).bg(color))
                    .height(1),
            )
        } else {
            None
        }
    });

    let bar = " █ ";
    Table::new(
        rows,
        [
            Constraint::Length(table.longest_item_lens[0]),
            Constraint::Min(table.longest_item_lens[1]),
            Constraint::Min(table.longest_item_lens[2]),
        ],
    )
    .header(header)
    .row_highlight_style(selected_style)
    .highlight_symbol(Text::from(vec![
        "".into(),
        bar.into(),
        bar.into(),
        "".into(),
    ]))
    .bg(table.colors.buffer_bg)
    .highlight_spacing(ratatui::widgets::HighlightSpacing::Always)
}

pub fn draw_session_table_ui(f: &mut Frame, table: &mut TableWidget) {
    table.active = true;
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(&[
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .split(f.area());

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(&[Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(vertical_chunks[1]);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(&[Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(vertical_chunks[2]);

    let main_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded);
    f.render_widget(main_block, f.area());

    let horizontal_line = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(Color::White));
    f.render_widget(horizontal_line, vertical_chunks[1]);

    let vertical_line = Block::default()
        .borders(Borders::LEFT)
        .border_style(Style::default().fg(Color::White));
    f.render_widget(
        vertical_line,
        Rect {
            x: f.area().width / 2,
            y: 1,
            width: 1,
            height: f.area().height - 2,
        },
    );

    let top_left_text = Paragraph::new(Line::from("Sessions").yellow())
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);
    f.render_widget(top_left_text, top_chunks[0]);

    let top_right_text = Paragraph::new(Line::from("Details".yellow()))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);
    f.render_widget(top_right_text, top_chunks[1]);

    // We assume `table` is already a mutable reference to the inner TableWidget.
    let mut table_state = std::mem::take(&mut table.state);
    let stateful_table = session_table_ui(table);
    f.render_stateful_widget(stateful_table, main_chunks[0], &mut table_state);

    let details_panel = session_details_ui(table);
    f.render_widget(details_panel, main_chunks[1]);

    table.state = table_state;
}
