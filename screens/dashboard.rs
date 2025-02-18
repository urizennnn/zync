use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table},
    Frame,
};

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

pub fn table_ui(f: &mut Frame, table: &mut TableWidget) {
    f.render_widget(Clear, f.area());
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(2),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(f.area());

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(vertical_chunks[1]);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
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

    let top_left_text = Paragraph::new(Line::from("Recent Transfers").yellow())
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);
    f.render_widget(top_left_text, top_chunks[0]);

    let top_right_text = Paragraph::new(Line::from("Activity Logs".yellow()))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);
    f.render_widget(top_right_text, top_chunks[1]);

    let mut table_state = std::mem::take(&mut table.state);
    let stateful_table = draw_table(table);
    f.render_stateful_widget(stateful_table, main_chunks[0], &mut table_state);
    table.state = table_state;
}

fn draw_table(table: &TableWidget) -> ratatui::widgets::Table<'_> {
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
