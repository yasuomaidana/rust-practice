use candle_datasets::vision::mnist;
use candle_nn::VarMap;
use crate::mnist_model::{training_loop, training_loop_cnn, LinearModel, Mlp, TrainingArgs, WhichModel};

pub(crate) fn get_mnist_model(
    args: &TrainingArgs,
    model_type: &WhichModel,
) {
    let m = mnist::load().unwrap();
    match model_type {
        WhichModel::Linear => training_loop::<LinearModel>(m, args),
        WhichModel::Mlp => {training_loop::<Mlp>(m, args)},
        WhichModel::Cnn => {training_loop_cnn(m, args)},
    }.expect("Failed to load model");

}