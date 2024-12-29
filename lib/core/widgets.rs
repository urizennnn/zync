use std::io;
use unicode_width::UnicodeWidthStr;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    style::{palette::tailwind, Color},
    text::Line,
    widgets::{ScrollbarState, TableState},
};

use crate::dashboard::dashboard_view::Data;
use crate::sessions::Device;

const ITEM_HEIGHT: usize = 2;
#[derive(Debug)]
pub enum Item {
    Data(Data),
    Device(Device),
}

#[derive(Debug)]
pub enum SelectedItem<'a> {
    Data(&'a Data),
    Device(&'a Device),
}
#[derive(Debug)]
pub struct TableWidget {
    pub state: TableState,
    pub items: Vec<Item>,
    pub longest_item_lens: Vec<u16>,
    pub scroll_state: ScrollbarState,
    pub colors: TableColors,
    pub help: bool,
    pub connection: bool,
    pub active: bool,
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

pub trait TableWidgetItemManager {
    type Item;

    fn add_item(&mut self, item: Self::Item, table: &mut TableWidget);
    fn constraint_len_calculator(items: &[&Self::Item]) -> Vec<u16>;
}

impl TableWidget {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            active: false,
            connection: false,
            state: TableState::default(),
            items: Vec::new(),
            longest_item_lens: vec![0; 4],
            scroll_state: ScrollbarState::default(),
            colors: TableColors::new(&tailwind::CYAN),
            help: false,
        }
    }

    pub async fn handle_events(&mut self) -> io::Result<()> {
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

    pub fn enter(&mut self) -> Option<SelectedItem> {
        if let Some(i) = self.state.selected() {
            match &self.items[i] {
                Item::Data(data) => Some(SelectedItem::Data(data)),
                Item::Device(device) => Some(SelectedItem::Device(device)),
            }
        } else {
            None
        }
    }
    pub fn add_item(
        &mut self,
        name: String,
        status: impl Into<Line<'static>>,
        destination: String,
        time: String,
    ) {
        self.items.push(Item::Data(Data {
            name,
            status: status.into(),
            destination,
            time,
        }));

        let data_items: Vec<&Data> = self
            .items
            .iter()
            .filter_map(|item| {
                if let Item::Data(data) = item {
                    Some(data)
                } else {
                    None
                }
            })
            .collect();

        self.longest_item_lens = Self::constraint_len_calculator(&data_items);
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

    fn constraint_len_calculator(items: &[&Data]) -> Vec<u16> {
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

        vec![
            name_percent,
            status_percent,
            destination_percent,
            time_percent,
        ]
    }
}
