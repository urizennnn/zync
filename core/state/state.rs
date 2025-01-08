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
