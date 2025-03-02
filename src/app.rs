use std::{
    error::Error,
    io::stdout,
    sync::{Arc, Mutex},
    time::Duration,
};

use color_eyre;
use crossterm::{
    event, execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use log;
use once_cell::sync::Lazy;
use ratatui::{Terminal, backend::CrosstermBackend};
use tokio::runtime::Runtime;
use tui_logger;

use crate::screens::home::Home;

pub static GLOBAL_RUNTIME: Lazy<Runtime> =
    Lazy::new(|| Runtime::new().expect("Failed to create tokio runtime"));

pub fn init_app() -> Result<(), Box<dyn Error>> {
    tui_logger::init_logger(log::LevelFilter::Trace)?;
    tui_logger::set_default_level(log::LevelFilter::Info);

    enable_raw_mode()?;
    color_eyre::install()?;

    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = restore_tui();
        original_hook(panic_info);
        eprintln!("Application panicked. Exiting.");
        std::process::exit(1);
    }));

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let terminal = Terminal::new(CrosstermBackend::new(stdout))?;
    let backend = Arc::new(Mutex::new(terminal));

    let (event_tx, event_rx) = std::sync::mpsc::channel();
    let event_thread = std::thread::spawn({
        let event_tx_clone = event_tx.clone();
        move || loop {
            if event::poll(Duration::from_millis(100)).unwrap() {
                match event::read() {
                    Ok(ev) => {
                        if event_tx_clone.send(ev).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        }
    });

    let app_result = Home::default().run(backend.clone(), event_rx);
    drop(event_tx);
    let _ = event_thread.join();

    if let Err(err) = backend.lock().unwrap().show_cursor() {
        let msg = format!("App panicked out: {:?}", err);
        restore_tui()?;
        panic!("{msg}");
    }

    restore_tui()?;
    app_result
}

fn restore_tui() -> std::io::Result<()> {
    let mut stdout = stdout();
    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}
