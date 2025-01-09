#[cfg(feature = "tui")]
use ratatui::{TerminalOptions, Viewport};

use snake::game::Game;
#[cfg(feature = "tui")]
use snake::init;

const GRID_HEIGHT: usize = 15;
const GRID_WIDTH: usize = 17;

#[cfg(feature = "tui")]
fn main() -> std::io::Result<()> {
    let terminal = init::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(GRID_HEIGHT as u16 + 2),
    });

    let game = Game::<GRID_WIDTH, GRID_HEIGHT>::default();

    let result = game.run(terminal);

    init::restore();
    result
}

#[cfg(not(feature = "tui"))]
fn main() -> std::io::Result<()> {
    let _game = Game::<GRID_WIDTH, GRID_HEIGHT>::default();

    Ok(())
}
