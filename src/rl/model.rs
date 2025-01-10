use burn::{
    module::Param,
    nn::{Linear, LinearConfig, Relu},
    prelude::{Backend, Config, Module, Tensor},
    tensor::backend::AutodiffBackend,
};

#[derive(Module, Debug)]
pub struct LinearQNet<B: Backend> {
    linear1: Linear<B>,
    linear2: Linear<B>,
    activation: Relu,
}

#[derive(Config, Debug)]
pub struct LinearQNetConfig {
    input_size: usize,
    hidden_size: usize,
    output_size: usize,
}

impl LinearQNetConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> LinearQNet<B> {
        LinearQNet {
            linear1: LinearConfig::new(self.input_size, self.hidden_size).init(device),
            linear2: LinearConfig::new(self.hidden_size, self.output_size).init(device),
            activation: Relu::new(),
        }
    }
}

impl<B: AutodiffBackend> LinearQNet<B> {
    pub fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        let x = self.linear1.forward(input);
        let x = self.activation.forward(x);

        self.linear2.forward(x)
    }

    pub fn soft_update(self, other: &Self, tau: f32) -> Self {
        Self {
            linear1: soft_update_linear(self.linear1, &other.linear1, tau),
            linear2: soft_update_linear(self.linear2, &other.linear2, tau),
            activation: self.activation,
        }
    }
}

fn soft_update_tensor<B: Backend, const D: usize>(
    this: Param<Tensor<B, D>>,
    that: &Param<Tensor<B, D>>,
    tau: f32,
) -> Param<Tensor<B, D>> {
    this.map(|tensor| tensor * (1.0 - tau) + that.val() * tau)
}

fn soft_update_linear<B: Backend>(mut this: Linear<B>, that: &Linear<B>, tau: f32) -> Linear<B> {
    this.weight = soft_update_tensor(this.weight, &that.weight, tau);
    this.bias = match (this.bias, &that.bias) {
        (Some(b1), Some(b2)) => Some(soft_update_tensor(b1, b2, tau)),
        _ => None,
    };

    this
}
