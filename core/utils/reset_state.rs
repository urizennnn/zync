use std::{collections::HashMap, fmt};

pub enum StateResetEnum {
    Home,
    HostType,
    ProtocolPopup,
    Connection,
}

pub struct StateReset {
    pub screens: HashMap<String, bool>,
}

impl fmt::Display for StateResetEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StateResetEnum::Home => write!(f, "home"),
            StateResetEnum::HostType => write!(f, "host_type"),
            StateResetEnum::ProtocolPopup => write!(f, "protocol_popup"),
            StateResetEnum::Connection => write!(f, "connection"),
        }
    }
}

impl StateReset {
    pub fn new() -> Self {
        StateReset {
            screens: HashMap::new(),
        }
    }

    pub fn reset_state(&mut self, except: Option<&str>) {
        for (screen, state) in self.screens.iter_mut() {
            if let Some(excluded) = except {
                if screen == excluded {
                    continue;
                }
            }
            *state = !*state;
        }
    }
    // TODO: find out their initial state
    pub fn init_state(&mut self) {
        self.screens.insert(StateResetEnum::Home.to_string(), true);
        self.screens
            .insert(StateResetEnum::HostType.to_string(), false);
        self.screens
            .insert(StateResetEnum::ProtocolPopup.to_string(), false);
        self.screens
            .insert(StateResetEnum::Connection.to_string(), false);
    }
}

impl Default for StateReset {
    fn default() -> Self {
        Self::new()
    }
}
