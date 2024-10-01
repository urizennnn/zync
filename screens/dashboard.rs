pub mod dashboard_view {

    use ratatui::{
        layout::{Alignment, Constraint, Direction, Layout, Rect},
        style::{palette::tailwind, Color, Modifier, Style, Stylize},
        text::{Line, Text},
        widgets::{Block, Borders, Cell, Paragraph, Row, ScrollbarState, Table, TableState},
        Frame,
    };
    use unicode_width::UnicodeWidthStr;

    #[derive(Debug)]
    pub struct TableWidget {
        state: TableState,
        items: Vec<Data>,
        longest_item_lens: (u16, u16, u16, u16),
        scroll_state: ScrollbarState,
        colors: TableColors,
    }

    const ITEM_HEIGHT: usize = 2;

    impl TableWidget {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            let items = vec![Data {
                name: "Name".to_string(),
                status: "Status".to_string(),
                destination: "Destination".to_string(),
                time: "Time".to_string(),
            }];

            Self {
                state: TableState::default(),
                longest_item_lens: Self::constraint_len_calculator(&items),
                scroll_state: ScrollbarState::default(),
                items: Vec::new(),
                colors: TableColors::new(&tailwind::CYAN),
            }
        }

        pub fn add_item(
            &mut self,
            name: String,
            status: String,
            destination: String,
            time: String,
        ) {
            self.items.push(Data {
                name,
                status,
                destination,
                time,
            });
            self.longest_item_lens = Self::constraint_len_calculator(&self.items);
        }

        pub fn next(&mut self) {
            let i = match self.state.selected() {
                Some(i) => {
                    if i >= self.items.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.state.select(Some(i));
            self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
        }

        pub fn previous(&mut self) {
            let i = match self.state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.items.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.state.select(Some(i));
            self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
        }

        fn constraint_len_calculator(items: &[Data]) -> (u16, u16, u16, u16) {
            let total_width: u16 = items
                .iter()
                .map(|item| {
                    item.name.width() as u16
                        + item.status.width() as u16
                        + item.destination.width() as u16
                        + item.time.width() as u16
                })
                .max()
                .unwrap_or(0);

            let name_percent = items
                .iter()
                .map(|item| item.name.width() as u16)
                .max()
                .unwrap_or(0)
                * 100
                / total_width;
            let status_percent = items
                .iter()
                .map(|item| item.status.width() as u16)
                .max()
                .unwrap_or(0)
                * 100
                / total_width;
            let destination_percent = items
                .iter()
                .map(|item| item.destination.width() as u16)
                .max()
                .unwrap_or(0)
                * 100
                / total_width;
            let time_percent = items
                .iter()
                .map(|item| item.time.width() as u16)
                .max()
                .unwrap_or(0)
                * 100
                / total_width;

            (
                name_percent,
                status_percent,
                destination_percent,
                time_percent,
            )
        }
    }

    pub fn ui(f: &mut Frame, table: &mut TableWidget) {
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

    #[derive(Debug)]
    struct Data {
        name: String,
        status: String,
        destination: String,
        time: String,
    }

    #[derive(Debug)]
    struct TableColors {
        buffer_bg: Color,
        header_bg: Color,
        header_fg: Color,
        row_fg: Color,
        selected_style_fg: Color,
        normal_row_color: Color,
        alt_row_color: Color,
    }

    impl TableColors {
        const fn new(color: &tailwind::Palette) -> Self {
            Self {
                //c700
                buffer_bg: tailwind::SLATE.c900,
                header_bg: color.c900,
                header_fg: tailwind::SLATE.c200,
                row_fg: tailwind::SLATE.c200,
                selected_style_fg: color.c400,
                normal_row_color: tailwind::SLATE.c950,
                alt_row_color: tailwind::SLATE.c900,
            }
        }
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

        let rows = table.items.iter().enumerate().map(|(i, data)| {
            let color = match i % 2 {
                0 => table.colors.normal_row_color,
                _ => table.colors.alt_row_color,
            };
            let item = [&data.name, &data.status, &data.destination, &data.time];
            item.into_iter()
                .map(|content| Cell::from(Text::from(content.to_string())))
                .collect::<Row>()
                .style(Style::new().fg(table.colors.row_fg).bg(color))
                .height(4)
        });

        let bar = " █ ";
        let t = Table::new(
            rows,
            [
                Constraint::Length(table.longest_item_lens.0),
                Constraint::Min(table.longest_item_lens.1),
                Constraint::Min(table.longest_item_lens.2),
                Constraint::Min(table.longest_item_lens.3),
            ],
        )
        .header(header)
        .highlight_style(selected_style)
        .highlight_symbol(Text::from(vec![
            "".into(),
            bar.into(),
            bar.into(),
            "".into(),
        ]))
        .bg(table.colors.buffer_bg)
        .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);
        t
    }
}
