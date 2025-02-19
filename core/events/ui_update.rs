use crate::state::state::ScreenState;
pub enum UIUpdate {
    ShowPopup(String),
    SwitchScreen(ScreenState),
}
