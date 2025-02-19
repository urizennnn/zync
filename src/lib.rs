#[path = "./app.rs"]
pub mod init;

#[path = "../config/mod.rs"]
pub mod config;

// #[path = "../lib/tcp/*"]
// pub mod tcp;
//

#[path = "../core/mod.rs"]
pub mod core_mod;

#[path = "../core/utils/mod.rs"]
pub mod utils;

#[path = "../core/events/mod.rs"]
pub mod events;
#[path = "../core/state/mod.rs"]
pub mod state;

#[path = "../screens/mod.rs"]
pub mod screens;

#[path = "../lib/internal/mod.rs"]
pub mod internal;
