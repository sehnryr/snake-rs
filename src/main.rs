mod apple;
mod game;
#[cfg(feature = "tui")]
mod init;
#[cfg(feature = "rl")]
mod model;
mod point;
mod snake;

#[cfg(feature = "rl")]
use std::sync::{LazyLock, Mutex};

#[cfg(all(feature = "tui", feature = "rl"))]
use ratatui::DefaultTerminal;
#[cfg(feature = "tui")]
use ratatui::{TerminalOptions, Viewport};

use crate::game::Game;

const GRID_HEIGHT: usize = 15;
const GRID_WIDTH: usize = 17;

#[cfg(all(feature = "tui", feature = "rl"))]
static TERMINAL: LazyLock<Mutex<DefaultTerminal>> = LazyLock::new(|| {
    Mutex::new(init::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(GRID_HEIGHT as u16 + 2),
    }))
});

#[cfg(all(feature = "tui", not(feature = "rl")))]
fn main() -> std::io::Result<()> {
    let terminal = init::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(GRID_HEIGHT as u16 + 2),
    });

    let game = Game::<GRID_WIDTH, GRID_HEIGHT>::default();

    let result = game.run(terminal);

    init::restore();
    result
}

#[cfg(all(not(feature = "tui"), feature = "rl"))]
fn main() {
    use rl::burn::backend::{wgpu::WgpuDevice, Wgpu};

    use rl::algo::dqn::{DQNAgent, DQNAgentConfig};
    use rl::burn::backend::Autodiff;

    use crate::model::LinearQNetConfig;

    type DQNBackend = Autodiff<Wgpu>;

    const NUM_EPISODES: u16 = 256;

    static DEVICE: LazyLock<WgpuDevice> = LazyLock::new(WgpuDevice::default);

    let mut env = Game::<GRID_WIDTH, GRID_HEIGHT>::default();

    let model = LinearQNetConfig::new(4, 256, 2).init::<DQNBackend>(&*DEVICE);
    let config = DQNAgentConfig::default();

    let mut dqn = DQNAgent::new(model, config, &*DEVICE);

    for _ in 0..NUM_EPISODES {
        dqn.go(&mut env);
        let report = env.report.take();
        println!("{:?}", report);
    }
}

#[cfg(all(feature = "tui", feature = "rl"))]
fn main() -> std::io::Result<()> {
    use rl::burn::backend::{wgpu::WgpuDevice, Wgpu};

    use rl::algo::dqn::{DQNAgent, DQNAgentConfig};
    use rl::burn::backend::Autodiff;

    use crate::model::LinearQNetConfig;

    type DQNBackend = Autodiff<Wgpu>;

    const NUM_EPISODES: u16 = 256;

    static DEVICE: LazyLock<WgpuDevice> = LazyLock::new(WgpuDevice::default);

    let mut env = Game::<GRID_WIDTH, GRID_HEIGHT>::default();

    let model = LinearQNetConfig::new(4, 256, 2).init::<DQNBackend>(&*DEVICE);
    let config = DQNAgentConfig::default();

    let mut dqn = DQNAgent::new(model, config, &*DEVICE);

    for _ in 0..NUM_EPISODES {
        dqn.go(&mut env);
    }

    init::restore();
    Ok(())
}

#[cfg(not(any(feature = "tui", feature = "rl")))]
fn main() {
    panic!("no enabled feature")
}
