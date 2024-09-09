use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::{
    error::Error,
    io::{self, stdout},
    panic::take_hook,
};
use tcshare_ui::home;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();

    enable_raw_mode()?;
    color_eyre::install()?;
    stdout.execute(EnterAlternateScreen)?;

    let tui = ratatui::init();
    let app = home::hello::Home::new().run(tui);
    restore_tui()?;
    app
}

pub fn install_hook() {
    let original_hook = take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        println!("Panic occurred: {:?}", panic_info);
        original_hook(panic_info);
    }));
}

pub fn restore_tui() -> io::Result<()> {
    let mut stdout = stdout();
    stdout.execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
