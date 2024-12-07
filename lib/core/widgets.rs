use std::io;
use unicode_width::UnicodeWidthStr;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    style::{palette::tailwind, Color},
    text::Line,
    widgets::{ScrollbarState, TableState},
};

use crate::dashboard::dashboard_view::Data;

#[derive(Debug)]
pub struct TableWidget {
    pub state: TableState,
    pub items: Vec<Data>,
    pub longest_item_lens: (u16, u16, u16, u16),
    pub scroll_state: ScrollbarState,
    pub colors: TableColors,
    pub help: bool,
    pub connection: bool,
}

const ITEM_HEIGHT: usize = 2;

impl TableWidget {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            connection: false,
            state: TableState::default(),
            items: Vec::new(),
            longest_item_lens: (0, 0, 0, 0),
            scroll_state: ScrollbarState::default(),
            colors: TableColors::new(&tailwind::CYAN),
            help: false,
        }
    }

    pub fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up => {
                    self.next();
                }
                KeyCode::Down => {
                    self.previous();
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn add_item(
        &mut self,
        name: String,
        status: impl Into<Line<'static>>,
        destination: String,
        time: String,
    ) {
        self.items.push(Data {
            name,
            status: status.into(),
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
#[derive(Debug)]
pub struct TableColors {
    pub buffer_bg: Color,
    pub header_bg: Color,
    pub header_fg: Color,
    pub row_fg: Color,
    pub selected_style_fg: Color,
    pub normal_row_color: Color,
    pub alt_row_color: Color,
}

impl TableColors {
    pub const fn new(color: &tailwind::Palette) -> Self {
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
