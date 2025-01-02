use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io::{self, stdout},
    panic::take_hook,
    sync::{Arc, Mutex},
};

use crate::screens::home::Home;

pub fn init_app() -> Result<(), Box<dyn Error>> {
    tui_logger::init_logger(log::LevelFilter::Trace)?;
    tui_logger::set_default_level(log::LevelFilter::Info);

    // Enable raw mode, color eyre for error handling, and setup panic hook
    enable_raw_mode()?;
    color_eyre::install()?;

    // Install custom panic hook
    let original_hook = take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        original_hook(panic_info);
        let (panic_hook, eyre_hook) = color_eyre::config::HookBuilder::default()
            .panic_section(format!(
                "This is a bug. Consider reporting it at {}",
                env!("CARGO_PKG_REPOSITORY")
            ))
            .display_location_section(true)
            .display_env_section(true)
            .into_hooks();
        eyre_hook.install().unwrap();
        let msg = format!("{}", panic_hook.panic_report(panic_info));
        eprintln!("{msg}");
        use human_panic::{handle_dump, print_msg, Metadata};
        let author = format!("authored by {}", env!("CARGO_PKG_AUTHORS"));
        let support = format!(
            "You can open a support request at {}",
            env!("CARGO_PKG_REPOSITORY")
        );
        let meta = Metadata::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
            .authors(author)
            .support(support);

        let file_path = handle_dump(&meta, panic_info);
        print_msg(file_path, &meta).expect("human-panic: printing error message to console failed");
        log::error!("Error: {}", strip_ansi_escapes::strip_str(msg));

        std::process::exit(libc::EXIT_FAILURE);
    }));

    // Start the application
    let mut stdout = stdout();
    crossterm::execute!(stdout, EnableMouseCapture, EnterAlternateScreen)?;

    let backend = Arc::new(Mutex::new(ratatui::init()));
    let app = Home::default().run(backend.clone());

    // Restore TUI state after app ends
    let res = backend.lock().unwrap().show_cursor();
    match res {
        Ok(_) => {}
        Err(err) => {
            let error_message = format!("App panicked out: {:?}", err);
            panic!("{error_message}")
        }
    }

    // Exit gracefully and restore terminal
    restore_tui()?;
    app
}

fn restore_tui() -> io::Result<()> {
    let mut stdout = stdout();
    crossterm::execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    disable_raw_mode()?;
    Ok(())
}
