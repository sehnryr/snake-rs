mod apple;
mod game;
mod point;
mod snake;

use ratatui::{TerminalOptions, Viewport};

use crate::game::Game;

const GRID_HEIGHT: u16 = 15;
const GRID_WIDTH: u16 = 17;

fn main() -> std::io::Result<()> {
    let terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(GRID_HEIGHT + 2),
    });

    let game = Game::default();

    let result = game.run(terminal);

    ratatui::restore();
    result
}
