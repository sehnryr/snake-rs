use std::collections::VecDeque;

use super::environment::Environment;

#[derive(Debug, Clone)]
pub struct Exp<E: Environment> {
    pub state: E::State,
    pub action: E::Action,
    pub reward: f32,
    pub next_state: Option<E::State>,
}

pub struct ReplayMemory<E: Environment> {
    memory: VecDeque<Exp<E>>,
    capacity: usize,
    batch_size: usize,
}

impl<E: Environment> ReplayMemory<E> {
    pub fn new(capacity: usize, batch_size: usize) -> Self {
        Self {
            memory: VecDeque::with_capacity(capacity),
            capacity,
            batch_size,
        }
    }

    pub fn push(&mut self, exp: Exp<E>) {
        if self.memory.len() == self.capacity {
            self.memory.pop_front();
        }
        self.memory.push_back(exp);
    }

    pub fn sample(&self) -> Vec<&Exp<E>> {
        fastrand::choose_multiple(&self.memory, self.batch_size)
    }
}
