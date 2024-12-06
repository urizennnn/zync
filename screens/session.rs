use ratatui::{style::Color, widgets::ScrollbarState};

pub struct DeviceTable {
    pub name: String,
    pub last_connection: String,
    pub status: String,
    pub longest_item_lens: (usize, usize, usize),
    pub scroll_state: ScrollbarState,
    pub colors: TableColors,
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
const ITEM_HEIGHT: usize = 3;
pub struct Device {
    pub name: String,
    pub ip: String,
    pub os: String,
    pub last_transfer: Transfer,
    pub last_connection: Connection,
}

pub struct Transfer {
    pub status: String,
    pub size: String,
    pub name: String,
}

pub struct Connection {
    pub total: String,
}

impl DeviceTable {}
