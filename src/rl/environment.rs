use std::fmt::Debug;

use burn::{
    prelude::Backend,
    tensor::{Element, Float, Int, Tensor, TensorData},
};

use super::to_tensor::ToTensor;

pub trait EnvState {
    fn to_vec(self) -> Vec<impl Element>;
}

impl<E: Element, const A: usize> EnvState for [E; A] {
    fn to_vec(self) -> Vec<impl Element> {
        self.into_iter().collect::<Vec<_>>()
    }
}

pub trait EnvAction: From<i32> + Into<i32> {}
impl<T: From<i32> + Into<i32>> EnvAction for T {}

pub trait Environment
where
    Self::State: Clone + Debug + EnvState,
    Self::Action: Clone + Debug + EnvAction,
{
    type State;
    type Action;

    fn step(&mut self, action: Self::Action) -> (Option<Self::State>, f32);

    fn reset(&mut self) -> Self::State;

    fn random_action(&self) -> Self::Action;

    fn is_active(&self) -> bool;

    fn actions(&self) -> Vec<Self::Action>;
}

impl<B, T> ToTensor<B, 2, Float> for Vec<T>
where
    B: Backend,
    T: EnvState,
{
    fn to_tensor(self) -> Tensor<B, 2, Float> {
        // let mut inner_data: Vec<TensorData> = Vec::with_capacity(self.len());
        // for elem in self {
        //     inner_data.push(elem.into());
        // }

        // let outer_dim = inner_data.len();
        // let inner_dim = inner_data
        //     .get(0)
        //     .map(TensorData::num_elements)
        //     .expect("cannot convert data with shape 0");

        // let dtype = inner_data.get(0).map(|x| x.dtype).unwrap();

        // let len = inner_data.get(0).map(|x| x.bytes.len()).unwrap();

        // let mut bytes = Vec::with_capacity(outer_dim * len);
        // for mut elem in inner_data {
        //     bytes.append(&mut elem.bytes);
        // }

        // Tensor::from(TensorData {
        //     bytes,
        //     shape: vec![outer_dim, inner_dim],
        //     dtype,
        // })

        let inner_data = self.into_iter().map(|x| x.to_vec()).collect::<Vec<_>>();

        let outer_dim = inner_data.len();
        let inner_dim = inner_data
            .get(0)
            .map(Vec::len)
            .expect("cannot convert data with shape 0");

        let data = TensorData::new(
            inner_data.into_iter().flatten().collect::<Vec<_>>(),
            vec![outer_dim * inner_dim],
        );

        let tensor: Tensor<B, 1, Float> = Tensor::from(data);
        tensor.reshape([-1, inner_dim as i32])
    }
}

impl<B, T> ToTensor<B, 2, Int> for Vec<T>
where
    B: Backend,
    T: EnvAction,
{
    fn to_tensor(self) -> Tensor<B, 2, Int> {
        let tensor: Tensor<B, 1, Int> = Tensor::from(
            self.into_iter()
                .map(|x| x.into())
                .collect::<Vec<i32>>()
                .as_slice(),
        );
        tensor.unsqueeze_dim(1)
    }
}
