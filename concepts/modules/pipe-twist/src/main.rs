use std::f64::consts::PI;
use custom_maths::function_evaluator;

fn circular_polar_moment_of_inertia(radius: f64) -> f64 {
    PI * radius.powi(4) / 32.0
}

/// Pipe polar moment of inertia
/// J = J_outer - J_inner
/// # Parameters
/// outer_radius: meters
/// inner_radius: meters
/// # Returns
/// J=meters^4
fn pipe_polar_moment_of_inertia(outer_radius: f64, inner_radius: f64) -> f64 {
    function_evaluator(circular_polar_moment_of_inertia, inner_radius, outer_radius)
}

fn shear_modulus(elasticity_modulus:f64, poison_ratio:f64)->f64{
    elasticity_modulus / (2.0 * (1.0 + poison_ratio))
}
fn main() {
    let length = 3.0;
    let outer_radius = 35.0/100.0;
    let thickness = 1.0/100.0;
    let inner_radius = outer_radius - thickness;

    let elasticity_modulus = 113e9;
    let poisons_ratio = 0.34;
    let shear_modulus = shear_modulus(elasticity_modulus, poisons_ratio);
    let torque = 100000.0;

    let polar_moment_of_inertia = pipe_polar_moment_of_inertia(outer_radius, inner_radius);

    let angle_of_twist = (torque * length) / (shear_modulus * polar_moment_of_inertia);
    println!("Angle of twist: {:.3} radians", angle_of_twist);


}
