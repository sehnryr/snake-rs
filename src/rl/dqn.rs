use burn::{
    config::Config,
    nn::loss::{MseLoss, Reduction},
    optim::{adaptor::OptimizerAdaptor, GradientsParams, Optimizer, SimpleOptimizer},
    tensor::{backend::AutodiffBackend, cast::ToElement, Tensor},
};

use super::memory::ReplayMemory;
use super::model::LinearQNet;
use super::to_tensor::ToTensor;
use super::{environment::Environment, memory::Exp};

#[derive(Config)]
pub struct DQNConfig {
    #[config(default = 16384)]
    pub memory_capacity: usize,
    #[config(default = 128)]
    pub memory_batch_size: usize,
    #[config(default = 0.99)]
    pub gamma: f32,
    #[config(default = 1e-3)]
    pub lr: f32,
    #[config(default = 0.9)]
    pub epsilon: f32,
    #[config(default = 5e-3)]
    pub tau: f32,
}

pub struct DQN<O, B, E>
where
    O: SimpleOptimizer<B::InnerBackend>,
    B: AutodiffBackend,
    E: Environment,
{
    policy_net: Option<LinearQNet<B>>,
    target_net: Option<LinearQNet<B>>,
    device: B::Device,
    memory: ReplayMemory<E>,
    optimizer: OptimizerAdaptor<O, LinearQNet<B>, B>,
    config: DQNConfig,
}

impl<O, B, E> DQN<O, B, E>
where
    O: SimpleOptimizer<B::InnerBackend>,
    B: AutodiffBackend,
    E: Environment,
{
    pub fn new(
        config: DQNConfig,
        model: LinearQNet<B>,
        optimizer: OptimizerAdaptor<O, LinearQNet<B>, B>,
        device: B::Device,
    ) -> Self {
        let policy_net = model;
        let target_net = policy_net.clone();

        let memory = ReplayMemory::new(config.memory_capacity, config.memory_batch_size);

        Self {
            policy_net: Some(policy_net),
            target_net: Some(target_net),
            device: device.clone(),
            memory,
            optimizer,
            config,
        }
    }

    fn act(&self, env: &E, state: E::State) -> E::Action {
        let random = fastrand::f32();

        if random > self.config.epsilon {
            env.random_action()
        } else {
            let t = vec![state];
            let input = t.to_tensor();
            let output = self
                .policy_net
                .as_ref()
                .unwrap()
                .forward(input)
                .argmax(1)
                .into_scalar();
            E::Action::from(output.to_i32())
        }
    }

    fn learn(&mut self) {
        let batch = self.memory.sample();
        let batch_size = self.config.memory_batch_size;

        let states: Vec<<E as Environment>::State> =
            batch.iter().map(|exp| exp.state.clone()).collect();
        let actions: Vec<<E as Environment>::Action> =
            batch.iter().map(|exp| exp.action.clone()).collect();
        let rewards: Vec<f32> = batch.iter().map(|exp| exp.reward).collect();
        let next_stages: Vec<Option<<E as Environment>::State>> =
            batch.iter().map(|exp| exp.next_state.clone()).collect();

        // Create a boolean mask for non-terminal next states so tensor shapes can match in the Bellman Equation
        let non_terminal_mask = next_stages
            .iter()
            .map(Option::is_some)
            .collect::<Vec<_>>()
            .to_tensor()
            .unsqueeze_dim(1);

        // Tensor conversions
        let states = states.to_tensor();
        let actions = actions.to_tensor();
        let next_states = next_stages
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .to_tensor();
        let rewards = rewards.to_tensor().unsqueeze_dim(1);

        let policy_net = self.policy_net.take().unwrap();
        let target_net = self.target_net.take().unwrap();

        // Compute the Q values of the chosen actions in each state
        let q_values = policy_net.forward(states).gather(1, actions);

        // Compute the maximum Q values obtainable from each next state
        let expected_q_values = Tensor::zeros([batch_size, 1], &self.device).mask_where(
            non_terminal_mask,
            target_net.forward(next_states).max_dim(1).detach(),
        );

        let discounted_expected_return = rewards + (expected_q_values * self.config.gamma);

        // Compute loss (mean sqared temporal difference error)
        let loss = MseLoss::new().forward(q_values, discounted_expected_return, Reduction::Mean);

        // Perform backpropagation on policy net
        let grads = GradientsParams::from_grads(loss.backward(), &policy_net);
        self.policy_net = Some(
            self.optimizer
                .step(self.config.lr.into(), policy_net, grads),
        );

        // Perform a periodic soft update on the parameters of the target network for stable convergence
        self.target_net =
            Some(target_net.soft_update(self.policy_net.as_ref().unwrap(), self.config.tau));
    }

    pub fn go(&mut self, env: &mut E) -> usize {
        let mut next_state = Some(env.reset());
        let mut steps = 0;

        while let Some(state) = next_state {
            let action = self.act(env, state.clone());
            let (next, reward) = env.step(action.clone());
            next_state = next;

            let exp = Exp {
                state,
                action,
                reward,
                next_state: next_state.clone(),
            };

            self.memory.push(exp);
            self.learn();

            steps += 1;
        }

        println!("{}", steps);

        steps
    }
}
