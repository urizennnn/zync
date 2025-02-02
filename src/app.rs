use crate::screens::home::Home;
use crossterm::{
    event::{self, DisableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::sync::{mpsc, Mutex};
use std::{
    error::Error,
    io::{self, stdout},
    panic::take_hook,
    sync::Arc,
    time::Duration,
};

pub async fn init_app() -> Result<(), Box<dyn Error>> {
    tui_logger::init_logger(log::LevelFilter::Trace)?;
    tui_logger::set_default_level(log::LevelFilter::Info);

    enable_raw_mode()?;
    color_eyre::install()?;

    let original_hook = take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = restore_tui();

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

    let mut stdout = stdout();
    crossterm::execute!(stdout, EnterAlternateScreen)?;

    let backend = Arc::new(Mutex::new(ratatui::init()));

    let (event_tx, event_rx) = mpsc::channel();

    let event_handle = tokio::spawn(async move {
        loop {
            if event::poll(Duration::from_millis(100))? {
                if let Ok(event) = event::read() {
                    if event_tx.send(event).is_err() {
                        break;
                    }
                }
            }
        }
        Ok::<_, io::Error>(())
    });

    let app_result = Home::default().run(backend.clone(), event_rx).await;

    event_handle.abort();
    let res = backend.lock().unwrap().show_cursor();
    if let Err(err) = res {
        let error_message = format!("App panicked out: {:?}", err);
        restore_tui()?;
        panic!("{error_message}");
    }

    restore_tui()?;
    app_result
}

fn restore_tui() -> io::Result<()> {
    let mut stdout = stdout();
    disable_raw_mode()?;
    crossterm::execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
