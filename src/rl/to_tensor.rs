use burn::{
    prelude::Backend,
    tensor::{BasicOps, Element, Tensor},
};

pub trait ToTensor<B, const D: usize, K>
where
    B: Backend,
    K: BasicOps<B>,
{
    fn to_tensor(self) -> Tensor<B, D, K>;
}

impl<B, E, K> ToTensor<B, 1, K> for Vec<E>
where
    B: Backend,
    E: Element,
    K: BasicOps<B>,
{
    fn to_tensor(self) -> Tensor<B, 1, K> {
        Tensor::from(self.as_slice())
    }
}
