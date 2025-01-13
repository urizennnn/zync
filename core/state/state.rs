use std::{cell::RefCell, rc::Rc, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    core_mod::widgets::TableWidget,
    screens::{
        connection_progress::ConnectionProgress, help::help_popup::HelpPopup, home::Home,
        host_type::HostTypePopup, popup::InputBox, protocol_popup::ConnectionPopup,
    },
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
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
#[derive(Clone)]
pub enum ConnectionState {
    Connecting,
    Connected,
    Failed(String),
}

pub struct StateSnapshot<'a> {
    pub home: &'a mut Home,
    pub table: &'a mut TableWidget,
    pub help: &'a mut HelpPopup,
    pub connection: &'a mut ConnectionPopup,
    pub input_box: &'a mut InputBox,
    pub host: &'a mut HostTypePopup,
    pub progress: &'a mut Arc<Mutex<ConnectionProgress>>,
}
