mod apple;
mod game;
#[cfg(feature = "tui")]
mod init;
#[cfg(feature = "rl")]
mod model;
mod point;
mod snake;

use crate::game::Game;

const GRID_HEIGHT: usize = 15;
const GRID_WIDTH: usize = 17;

#[cfg(feature = "rl")]
const NUM_EPISODES: u16 = 256;

#[cfg(all(feature = "tui", not(feature = "rl")))]
fn main() -> std::io::Result<()> {
    use ratatui::{TerminalOptions, Viewport};

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
    use burn::backend::{wgpu::WgpuDevice, Autodiff, Wgpu};
    use rl::{
        algo::dqn::{DQNAgent, DQNAgentConfig},
        viz,
    };

    use crate::model::ModelConfig;

    let device = WgpuDevice::default();

    let mut env = Game::<GRID_WIDTH, GRID_HEIGHT>::default();

    let model = ModelConfig::new(64, 128).init::<Autodiff<Wgpu>>(&device);
    let agent_config = DQNAgentConfig::default();
    let mut agent = DQNAgent::new(model, agent_config, &device);

    let (handle, tx) = viz::init(env.report.keys(), NUM_EPISODES);

    for i in 0..NUM_EPISODES {
        agent.go(&mut env);
        let report = env.report.take();
        tx.send(viz::Update {
            episode: i,
            data: report.values().copied().collect(),
        })
        .unwrap()
    }

    let _ = handle.join();
}

#[cfg(all(feature = "tui", feature = "rl"))]
fn main() -> std::io::Result<()> {
    todo!("load model to play the tui snake")
}

#[cfg(not(any(feature = "tui", feature = "rl")))]
fn main() {
    panic!("no enabled feature");
}
