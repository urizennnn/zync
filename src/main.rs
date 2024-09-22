use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::{
    error::Error,
    io::{self, stdout},
    panic::{take_hook, PanicHookInfo},
};
use tcshare_ui::home;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    tui_logger::init_logger(log::LevelFilter::Trace)?;
    tui_logger::set_default_level(log::LevelFilter::Info);

    enable_raw_mode()?;
    color_eyre::install()?;
    install_hook();
    stdout.execute(EnterAlternateScreen)?;

    let tui = ratatui::init();
    let app = home::homepage::Home::default().run(tui);
    restore_tui()?;
    app
}

pub fn install_hook() {
    let original_hook = take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        println!("Panic occurred: {:?}", panic_info);
        initialize_panic_handler(panic_info).unwrap();
        original_hook(panic_info);
    }));
}

pub fn restore_tui() -> io::Result<()> {
    let mut stdout = stdout();
    stdout.execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub fn initialize_panic_handler(
    panic_info: &PanicHookInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    let (panic_hook, eyre_hook) = color_eyre::config::HookBuilder::default()
        .panic_section(format!(
            "This is a bug. Consider reporting it at {}",
            env!("CARGO_PKG_REPOSITORY")
        ))
        .display_location_section(true)
        .display_env_section(true)
        .into_hooks();
    eyre_hook.install()?;
    let msg = format!("{}", panic_hook.panic_report(panic_info));
    #[cfg(not(debug_assertions))]
    {
        eprintln!("{}", msg); // prints color-eyre stack trace to stderr
        use human_panic::{handle_dump, print_msg, Metadata};
        let meta = Metadata {
            version: env!("CARGO_PKG_VERSION").into(),
            name: env!("CARGO_PKG_NAME").into(),
            authors: env!("CARGO_PKG_AUTHORS").replace(':', ", ").into(),
            homepage: env!("CARGO_PKG_HOMEPAGE").into(),
        };

        let file_path = handle_dump(&meta, panic_info);
        // prints human-panic message
        print_msg(file_path, &meta).expect("human-panic: printing error message to console failed");
    }
    log::error!("Error: {}", strip_ansi_escapes::strip_str(msg));

    #[cfg(debug_assertions)]
    {
        better_panic::Settings::auto()
            .most_recent_first(false)
            .lineno_suffix(true)
            .verbosity(better_panic::Verbosity::Full)
            .create_panic_handler()(panic_info);
    }

    std::process::exit(libc::EXIT_FAILURE);
    Ok(())
}
