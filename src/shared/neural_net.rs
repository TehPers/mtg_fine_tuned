use ndarray::{Array1, Array2};
use rayon::array;

type Val = f64;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct NeuralNetwork<A> {
    pub activation: A,
}

impl<A> NeuralNetwork<A>
where
    A: Fn(Val) -> Val,
{
    pub fn forward(&mut self, mut input: Array1<Val>) {
        
    }
}

pub struct Layer {
    weights: Array2<Val>,
    bias: Array1<Val>,
}

impl Layer {
    pub fn new_rand() -> Self {
        todo!()
    }
}

pub fn sigmoid(x: Array1<Val>) -> Array1<Val> {
    1.0 / (1.0 + (-x).map(|&n| n.exp()))
}
