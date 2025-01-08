mod apple;
mod game;
mod init;
mod point;
mod snake;

use ratatui::{TerminalOptions, Viewport};

use crate::game::Game;

const GRID_HEIGHT: usize = 15;
const GRID_WIDTH: usize = 17;

fn main() -> std::io::Result<()> {
    let terminal = init::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(GRID_HEIGHT as u16 + 2),
    });

    let game = Game::<GRID_WIDTH, GRID_HEIGHT>::default();

    let result = game.run(terminal);

    init::restore();
    result
}
