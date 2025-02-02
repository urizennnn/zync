#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ScreenState {
    Home,
    ConnectionLog,
    Popup,
    Transfer,
    Connection,
    Help,
    TCP,
    Sessions,
    TcpLogs,
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
    pub progress: &'a mut ConnectionProgress,
}
