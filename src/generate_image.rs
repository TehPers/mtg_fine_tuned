mod shared;

use linfa::prelude::*;
use linfa_linear::TweedieRegressor;
use ndarray::{Array, Array3};
use shared::neural_net::NeuralNetwork;

fn main() -> anyhow::Result<()> {
    let dataset = linfa_datasets::diabetes();
    let lin_reg = TweedieRegressor::params().power(0.0).alpha(0.0);
    let model = lin_reg.fit(&dataset)?;

    Ok(())
}

fn init_image(rows: usize, cols: usize) -> Array3<f64> {
    Array::zeros((rows, cols, 4))
}

fn prep_nnet_arch<A>(
    depth: usize,
    size: usize,
    activation: A,
    color_mode: &str,
) -> NeuralNetwork<A> {
    todo!()
}
