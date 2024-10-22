use std::f32::consts::PI;
use custom_maths::function_evaluator;

fn main() {
    // Parabola function y = x^2
    // Volume of revolution around y-axis
    // r = x = sqrt(y)
    // V = π ∫[a, b] r^2 dy
    // V = π ∫[a, b] y dy
    let fv = |y: f32| -> f32 { PI * y.powf(2.0) / 2.0 };
    let v = function_evaluator(fv, 0.0, 4.0);
    println!("Volume of revolution around y-axis: {:.2}", v);
    // Convert inches^3 to ounces
    let ounces = v * 0.554112554;
    println!("Volume in ounces: {:.2}", ounces);
}
