mod apple;
mod game;
#[cfg(feature = "tui")]
mod init;
mod point;
#[cfg(feature = "rl")]
mod rl;
mod snake;

#[cfg(feature = "tui")]
use ratatui::{TerminalOptions, Viewport};

use crate::game::Game;
#[cfg(feature = "tui")]
use crate::init;

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

#[cfg(feature = "rl")]
fn main() {
    #[cfg(feature = "ndarray")]
    use burn::backend::{ndarray::NdArrayDevice, NdArray};
    #[cfg(feature = "wgpu")]
    use burn::backend::{wgpu::WgpuDevice, Wgpu};

    use burn::{backend::Autodiff, optim::AdamConfig};

    use crate::rl::{
        dqn::{DQNConfig, DQN},
        model::LinearQNetConfig,
    };

    #[cfg(feature = "ndarray")]
    type Backend = Autodiff<NdArray>;
    #[cfg(feature = "wgpu")]
    type Backend = Autodiff<Wgpu>;

    const NUM_EPISODES: u16 = 256;

    let mut env = Game::<GRID_WIDTH, GRID_HEIGHT>::default();

    #[cfg(feature = "ndarray")]
    let device = NdArrayDevice::default();
    #[cfg(feature = "wgpu")]
    let device = WgpuDevice::default();

    let model = LinearQNetConfig::new(4, 256, 2).init::<Backend>(&device);
    let config = DQNConfig::new();
    let optimizer = AdamConfig::new().with_epsilon(config.epsilon).init();

    let mut dqn = DQN::new(config, model, optimizer, device);

    for _ in 0..NUM_EPISODES {
        dqn.go(&mut env);
    }
}

#[cfg(not(any(feature = "tui", feature = "rl")))]
fn main() {
    panic!("no enabled feature")
}
