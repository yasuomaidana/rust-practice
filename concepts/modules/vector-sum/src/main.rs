mod vector;

use vector::Vector;
use crate::vector::Angle;

fn main() {
    let vector1 = Vector::new(0.0, 20.0);

    let vector2 = Vector::from_ratio(30.0, 3.0, 4.0);

    let angle3 = Angle::new(-30.0);
    let vector3 = Vector::from_angle(40.0, angle3);

    let angle4 = Angle::new(-90.0 - 15.0);
    let vector4 = Vector::from_angle(30.0, angle4);

    let vector_sum = vector1.add(&vector2).add(&vector3).add(&vector4);
    println!("Vector sum: {:.2?}, magnitude: {:.2} lb angle: {:.2}°",
             vector_sum, vector_sum.mag(), vector_sum.angle());
}
