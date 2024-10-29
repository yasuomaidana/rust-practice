use std::f64::consts::PI;

fn main() {
    let load = 25.0; // kg
    let load_force = 9.81 * load; // N

    let outer_radius = 10.0 / 2.0; // cm
    let inner_radius = 9.6 / 2.0; // cm
    let outer_radius_m: f64 = outer_radius / 100.0; // m
    let inner_radius_m: f64 = inner_radius / 100.0; // m

    let area = PI * (outer_radius_m.powi(2) - inner_radius_m.powi(2));
    println!("Area: {} m²", &area);
    let pressure = load_force / area;

    let young_modulus = 75000.0e6; // Pa
    let length_initial = 1.2; // m

    let strain = pressure / young_modulus;
    let deformation = strain * length_initial;
    println!("Deformation: {} m", &deformation);
    println!("Deformation: {} mm", &deformation * 1000.0);

    let shear_force = 9600.0; // N
    let area = 6.0 * 9.0;
    let shear_stress = shear_force / area;
    println!("Shear stress: {:.2} Pa", &shear_stress);


}
