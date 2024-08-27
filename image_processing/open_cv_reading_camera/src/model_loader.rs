use candle_core::{DType, Device};
use candle_core::utils::metal_is_available;
use candle_datasets::vision::mnist;
use candle_nn::{VarBuilder, VarMap};
use crate::mnist_model::{training_loop, training_loop_cnn, Model, TrainingArgs, WhichModel};

pub(crate) fn get_mnist_model<M: Model + 'static>(
    args: &TrainingArgs,
    model_type: &WhichModel,
) -> Result<(M, Device), ()> {
    let m = mnist::load().unwrap();
    let mut varmap = VarMap::new();

    let metal_available = metal_is_available();
    let dev = if metal_available {
        Device::new_metal(0).expect("Failed to create metal device")
    } else {
        Device::cuda_if_available(0).expect("Failed to create cuda device")
    };


    if let Some(load) = &args.load {
        println!("loading weights from {}", load);
        varmap.load(load).expect("Failed to load weights");
        let vs = VarBuilder::from_varmap(&varmap, DType::F32, &dev);
        let model = M::new(vs).expect("Failed to create model");
        return Ok((model,dev))
    }

    match model_type {
        WhichModel::Cnn => {training_loop_cnn(m, args)}.expect("Failed to train model"),
        _ => {training_loop::<M>(m, args)}.expect("Failed to train model"),
    };
    let args_2 = TrainingArgs {
        learning_rate: args.learning_rate,
        load: args.save.clone(),
        save: args.save.clone(),
        epochs: args.epochs,
    };
    get_mnist_model::<M>(&args_2, model_type)
}