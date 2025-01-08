use std::io::stdout;

use ratatui::crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::prelude::CrosstermBackend;
use ratatui::{DefaultTerminal, Terminal, TerminalOptions};

pub fn init_with_options(options: TerminalOptions) -> DefaultTerminal {
    try_init_with_options(options).expect("failed to initialize terminal")
}

pub fn try_init_with_options(options: TerminalOptions) -> std::io::Result<DefaultTerminal> {
    set_panic_hook();
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout());
    Terminal::with_options(backend, options)
}

pub fn restore() {
    if let Err(err) = try_restore() {
        eprintln!("Failed to restore terminal: {err}");
    }
}

pub fn try_restore() -> std::io::Result<()> {
    disable_raw_mode()?;
    Ok(())
}

fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        restore();
        hook(info);
    }));
}
