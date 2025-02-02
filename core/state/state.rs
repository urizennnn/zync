use std::sync::Arc;

use crate::{
    core_mod::widgets::TableWidget,
    screens::{
        connection_progress::ConnectionProgress, help::help_popup::HelpPopup,
        host_type::HostTypePopup, popup::InputBox, protocol_popup::ConnectionPopup,
    },
};
use std::sync::Mutex;

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
}
#[derive(Clone, Debug)]
pub enum ConnectionState {
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
}
