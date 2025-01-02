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
}
