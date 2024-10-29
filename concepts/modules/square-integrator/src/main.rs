use custom_maths::function_evaluator;

fn main() {

    let distance_f = |x: f64| 4.0 * x - 0.2*x.powi(2)/2.0;
    let seconds = 36.0;
    let minutes =  15.0 + seconds / 60.0;
    let hours = 8.0 + minutes / 60.0;
    let distance = function_evaluator(distance_f, 0.0, hours);
    println!("Traveled distance is {:.3} m", distance);
}
