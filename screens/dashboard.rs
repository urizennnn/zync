pub mod dashboard_view {

    use ratatui::{
        layout::{Alignment, Constraint, Direction, Layout, Rect},
        style::{palette::tailwind, Color, Modifier, Style, Stylize},
        text::Line,
        widgets::{Block, Borders, Cell, Paragraph, Row, ScrollbarState, TableState},
        Frame,
    };
    use unicode_width::UnicodeWidthStr;
    #[derive(Debug)]
    struct Table {
        state: TableState,
        items: Vec<Data>,
        longest_item_lens: (u16, u16, u16, u16),
        scroll_state: ScrollbarState,
        colors: TableColors,
        color_index: usize,
    }

    const ITEM_HEIGHT: usize = 2;
    impl Table {
        fn new() -> Self {
            Self {
                state: TableState::default(),
                items: vec![],
                longest_item_lens: (0, 0, 0, 0),
                scroll_state: ScrollbarState::default(),
                colors: TableColors::new(&tailwind::BLUE),
                color_index: 0,
            }
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
            let name = Data::name;
            let status = Data::status;
            let destination = Data::destination;
            let time = Data::time;
            let name_len = items
                .iter()
                .map(|item| name(item).width() as u16)
                .max()
                .unwrap_or(0);
            let status_len = items
                .iter()
                .map(|item| status(item).width() as u16)
                .max()
                .unwrap_or(0);
            let destination_len = items
                .iter()
                .map(|item| destination(item).width() as u16)
                .max()
                .unwrap_or(0);
            let time_len = items
                .iter()
                .map(|item| time(item).width() as u16)
                .max()
                .unwrap_or(0);
            (name_len, status_len, destination_len, time_len)
        }
    }
    pub fn ui(f: &mut Frame) {
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
    }

    #[derive(Debug)]
    struct Data {
        name: String,
        status: String,
        destination: String,
        time: String,
    }

    impl Data {
        const fn ref_array(&self) -> [&String; 4] {
            [&self.name, &self.status, &self.destination, &self.time]
        }
        fn name(&self) -> &String {
            &self.name
        }
        fn status(&self) -> &String {
            &self.status
        }
        fn destination(&self) -> &String {
            &self.destination
        }
        fn time(&self) -> &String {
            &self.time
        }
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
        footer_border_color: Color,
    }

    impl TableColors {
        const fn new(color: &tailwind::Palette) -> Self {
            Self {
                buffer_bg: tailwind::SLATE.c950,
                header_bg: color.c900,
                header_fg: tailwind::SLATE.c200,
                row_fg: tailwind::SLATE.c200,
                selected_style_fg: color.c400,
                normal_row_color: tailwind::SLATE.c950,
                alt_row_color: tailwind::SLATE.c900,
                footer_border_color: color.c400,
            }
        }
    }
    fn draw_table(table: &Table, data: Data) {
        let header_style = Style::default()
            .fg(table.colors.header_fg)
            .bg(table.colors.header_bg);
        let selected_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(table.colors.selected_style_fg);
        let header = ["Name", "Status", "Destination", "Time"]
            .iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);
    }
}
