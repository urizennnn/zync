use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, Wrap},
    Frame,
};
use std::sync::{Arc, Mutex};

use crate::core_mod::widgets::{Item, TableWidget};

#[derive(Debug, Clone)]
pub struct Data {
    pub name: String,
    pub status: Line<'static>,
    pub destination: String,
    pub time: String,
}

#[derive(Debug)]
pub struct Activity {
    pub name: String,
    pub status: Line<'static>,
    pub destination: String,
    pub time: String,
}

/// Draws the transfers screen and, at the top–right, renders the current connection status.
/// Note the added parameter `progress` (an Arc–wrapped Mutex over your ConnectionProgress).
pub fn table_ui(
    f: &mut Frame,
    table: &mut TableWidget,
    progress: Arc<Mutex<crate::screens::connection_progress::ConnectionProgress>>,
) {
    // Clear the entire frame
    f.render_widget(Clear, f.area());

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

    // Main border around the transfers area
    let main_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded);
    f.render_widget(main_block, f.area());

    // Horizontal line for separation
    let horizontal_line = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(Color::White));
    f.render_widget(horizontal_line, vertical_chunks[1]);

    // Vertical line for separation
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

    // "Recent Transfers" label (top–left)
    let top_left_text = Paragraph::new(Line::from("Recent Transfers").yellow())
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);
    f.render_widget(top_left_text, top_chunks[0]);

    // "Activity Logs" label (top–right)
    let top_right_text = Paragraph::new(Line::from("Activity Logs").yellow())
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);
    f.render_widget(top_right_text, top_chunks[1]);

    // Render the table
    let mut table_state = std::mem::take(&mut table.state);
    let stateful_table = draw_table(table);
    f.render_stateful_widget(stateful_table, main_chunks[0], &mut table_state);
    table.state = table_state;

    // --- New: Render the connection status label in the top–right corner ---
    let connect_state = {
        let lock = progress.lock().unwrap();
        lock.state.clone()
    };
    let (status_str, color) = match connect_state {
        crate::state::state::ConnectionState::Connected => ("CONNECTED", Color::Green),
        crate::state::state::ConnectionState::NoConnection => ("NO CONNECTION", Color::Red),
        crate::state::state::ConnectionState::Connecting => ("CONNECTING", Color::Yellow),
        crate::state::state::ConnectionState::Failed(_) => ("FAILED", Color::Red),
    };

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
    ]))
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true });

    f.render_widget(status_text, corner_rect);
}

/// Helper: Draws the inner table of transfers.
fn draw_table(table: &TableWidget) -> Table<'_> {
    let header_style = Style::default()
        .fg(table.colors.header_fg)
        .bg(table.colors.header_bg);
    let selected_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(table.colors.selected_style_fg);
    let header = ["Name", "Status", "Destination", "Time"]
        .iter()
        .map(|&s| Cell::from(s))
        .collect::<Row>()
        .style(header_style)
        .height(1);

    let rows = table.items.iter().enumerate().filter_map(|(i, data)| {
        if let Item::Data(data) = data {
            let color = match i % 2 {
                0 => table.colors.normal_row_color,
                _ => table.colors.alt_row_color,
            };

            let cells = vec![
                Cell::from(data.name.clone()),
                Cell::from(data.status.clone()),
                Cell::from(data.destination.clone()),
                Cell::from(data.time.clone()),
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
            Constraint::Min(table.longest_item_lens[3]),
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
