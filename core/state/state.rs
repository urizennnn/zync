use crate::{
    core_mod::widgets::TableWidget,
    screens::{
        connection_progress::ConnectionProgress, debug::DebugScreen, help::help_popup::HelpPopup,
        host_type::HostTypePopup, popup::InputBox, protocol_popup::ConnectionPopup,
    },
};
use std::sync::{Arc, Mutex};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub enum ScreenState {
    Home,
    Popup,
    Transfer,
    Connection,
    Help,
    TCP,
    Sessions,
    TcpServer,
    TcpClient,
    Debug,
}
#[derive(Clone, Debug)]
pub enum ConnectionState {
    NoConnection,
    Connecting,
    Connected,
    Failed(String),
}

pub struct StateSnapshot {
    pub table: Arc<Mutex<TableWidget>>,
    pub help: Arc<Mutex<HelpPopup>>,
    pub connection: Arc<Mutex<ConnectionPopup>>,
    pub input_box: Arc<Mutex<InputBox>>,
    pub host: Arc<Mutex<HostTypePopup>>,
    pub progress: Arc<Mutex<ConnectionProgress>>,
    pub debug_screen: Arc<Mutex<DebugScreen>>,
    pub stream: bool,
}
