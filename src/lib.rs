#[path = "./app.rs"]
pub mod init;

#[path = "../screens/home.rs"]
pub mod home;

#[path = "../screens/popup.rs"]
pub mod popup;

#[path = "../lib/core/core.rs"]
pub mod core;

#[path = "../config/app.rs"]
pub mod app;

#[path = "../screens/dashboard.rs"]
pub mod dashboard;

#[path = "../screens/help.rs"]
pub mod help;

// #[path = "../lib/tcp/*"]
// pub mod tcp;
//
#[path = "../screens/protocol_popup.rs"]
pub mod protocol;

#[path = "../screens/error.rs"]
pub mod error;

#[path = "../screens/session.rs"]
pub mod sessions;

#[path = "../lib/core/widgets.rs"]
pub mod widget;

#[path = "../lib/core/event.rs"]
pub mod event;

#[path = "../lib/core/events/input.rs"]
pub mod input;

#[path = "../lib/core/utils/calculate.rs"]
pub mod calculate;

#[path = "../lib/core/state/state.rs"]
pub mod state;

#[path = "../lib/core/state/manager.rs"]
pub mod manager;
