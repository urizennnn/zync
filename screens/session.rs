use crate::core_mod::widgets::{Item, TableWidget};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::Stylize,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};
use std::sync::{Arc, Mutex};

use crate::screens::connection_progress::ConnectionProgress;
use crate::screens::dashboard::Data;
use crate::state::state::ConnectionState;

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

pub fn session_table_ui(table: &mut TableWidget) -> Table<'_> {
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

/// NOTE the new `progress` parameter:
pub fn draw_session_table_ui(
    f: &mut Frame,
    table: &mut TableWidget,
    progress: Arc<Mutex<ConnectionProgress>>,
) {
    table.active = true;
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .split(f.area());

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(vertical_chunks[1]);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(vertical_chunks[2]);

    // Main border
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

    // “Sessions” label (top-left)
    let top_left_text = Paragraph::new(Line::from("Sessions"))
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center);
    f.render_widget(top_left_text, top_chunks[0]);

    // “Details” label (top-right)
    let top_right_text = Paragraph::new(Line::from("Details"))
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center);
    f.render_widget(top_right_text, top_chunks[1]);

    // Render your table on the left side
    let mut table_state = std::mem::take(&mut table.state);
    let stateful_table = session_table_ui(table);
    f.render_stateful_widget(stateful_table, main_chunks[0], &mut table_state);
    table.state = table_state;

    // Render your “details” box on the right side
    let details_panel = session_details_ui(table);
    f.render_widget(details_panel, main_chunks[1]);

    // === NEW LOGIC: “progress” usage ===
    let connect_state = {
        // Safely lock the `progress` to read the ConnectionState
        let lock = progress.lock().unwrap();
        lock.state.clone()
    };

    let (status_str, color) = match connect_state {
        ConnectionState::Connected => ("CONNECTED", Color::Green),
        ConnectionState::NoConnection => ("NO CONNECTION", Color::Red),
        ConnectionState::Connecting => ("CONNECTING", Color::Yellow),
        ConnectionState::Failed(_) => ("FAILED", Color::Red),
    };

    // Draw the status in the top-right corner
    let corner_rect = Rect {
        x: f.area().width.saturating_sub(18),
        y: 0,
        width: 18,
        height: 1,
    };

    let status_text = Paragraph::new(Line::from(vec![
        Span::raw("Status: "),
        Span::styled(
            status_str,
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ),
    ]));

    f.render_widget(status_text, corner_rect);
}
