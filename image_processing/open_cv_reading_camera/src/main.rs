mod mnist_model;
mod model_loader;

use candle_core::{DType, Device, Tensor, D};
use candle_nn::ops::{softmax};
use ndarray::{ArrayBase, Dim, Ix, OwnedRepr};
use opencv::{Result, prelude::*, videoio, highgui, imgproc};
use opencv::core::Size;
use opencv::imgproc::{COLOR_BGR2HSV, INTER_LINEAR};
use crate::mnist_model::{ConvNet, Model, TrainingArgs, WhichModel};
use crate::model_loader::get_mnist_model;


fn mat_to_ndarray(mat: &Mat) -> Result<ArrayBase<OwnedRepr<f32>, Dim<[Ix; 2]>>, opencv::Error> {
    let shape = [mat.rows() as usize, mat.cols() as usize];
    let mut array = ndarray::Array::zeros(shape);

    for i in 0..mat.rows() {
        for j in 0..mat.cols() {
            // Use `?` to propagate errors from `at_2d`
            array[[i as usize, j as usize]] = *mat.at_2d::<u8>(i, j)? as f32;
        }
    }
    Ok(array)
}

fn mat_to_tensor(mat: &Mat, device: &Device) -> Result<Tensor, Box<dyn std::error::Error>> {
    let array = mat_to_ndarray(mat)?;
    let flattened_array = array.to_shape((28,28)).unwrap();
    let tensor = Tensor::from_vec(
        flattened_array.iter().map(|x| *x).collect(),
        &[28*28],
        device
    ).unwrap();
    Ok(tensor)
}

fn main() -> Result<()> {

    let args = TrainingArgs {
        learning_rate: 0.01,
        load: Some("cnn_mlp_mnist_model.safetensors".to_string()),
        save: Some("cnn_mlp_mnist_model.safetensors".to_string()),
        epochs: 5,
    };

    let model_type = WhichModel::Cnn; // Choose your model type

    let (model, device) = get_mnist_model::<ConvNet>(&args, &model_type).expect("Failed to load");



    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    highgui::named_window("Blue filtered", highgui::WINDOW_AUTOSIZE)?;
    let mut frame = Mat::default();
    let mut hsv = Mat::default();
    let mut mask = Mat::default();

    let lower_blue = opencv::core::Scalar::new(75.0, 25.0, 100.0, 0.0);
    let upper_blue = opencv::core::Scalar::new(140.0, 255.0, 255.0, 0.0);

    loop {
        // Read frame from camera
        cam.read(&mut frame)?;
        // Convert frame to HSV
        imgproc::cvt_color(&frame, &mut hsv, COLOR_BGR2HSV, 0)?;
        // Filter out blue color
        opencv::core::in_range(&hsv, &lower_blue, &upper_blue, &mut mask)?;

        // Apply mask to frame
        let mut blue_only_frame = Mat::default();
        frame.copy_to_masked(&mut blue_only_frame, &mask)?;
        let mut gray = Mat::default();
        imgproc::cvt_color(&blue_only_frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
        let mut resized_gray = Mat::default();
        imgproc::resize(&gray, &mut resized_gray, Size::new(28, 28), 0.0, 0.0, INTER_LINEAR)?;

        let tensor = mat_to_tensor(&resized_gray, &device).unwrap();
        let tensor = tensor.unsqueeze(0).unwrap();
        let result = model.forward(&tensor).unwrap();
        let flattened_result = softmax(&result.flatten_all().unwrap(), D::Minus1).unwrap().flatten_all().unwrap().to_vec1::<f32>().unwrap();
        let guessing = result.argmax(D::Minus1).unwrap().to_dtype(DType::U32).unwrap().to_vec1::<u32>().unwrap();
        println!("{:?}", guessing);
        println!("{:?}", flattened_result);
        println!("{:?}", result.argmax(D::Minus1).unwrap().to_dtype(DType::U32).unwrap().sum_all().unwrap().to_scalar::<u32>().unwrap());

        highgui::imshow("Blue filtered", &blue_only_frame)?;

        let key = highgui::wait_key(10)?;
        match key {
            key if key == 'q' as i32 => break,
            _ => ()
        }
    }
    Ok(())
}
