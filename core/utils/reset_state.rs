use std::collections::HashMap;

pub struct StateReset {
    pub screens: HashMap<String, bool>,
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
            *state = false;
        }
    }
    // TODO: find out their initial state
    pub fn init_state(&mut self) {
        self.screens.insert("home".to_string(), true);
        self.screens.insert("host_type".to_string(), false);
        self.screens.insert("protocol_popup".to_string(), false);
    }
}

impl Default for StateReset {
    fn default() -> Self {
        Self::new()
    }
}
