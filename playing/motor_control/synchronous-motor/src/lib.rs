/// Calculates the synchronous-motor speed of a motor.
///
/// # Arguments
///
/// * `f` - Line frequency Hz
/// * `p` - The number of poles of the motor.
///
/// # Returns
///
/// * `N_s` The synchronous-motor speed of the motor in RPM.
///
/// # Formula
///
/// The formula used to calculate the synchronous-motor speed \( N_s \) is:
/// $N_s = 120*f/p$
///
pub fn synchronous_speed(f: f32, p: u8) -> f32 {
    if p < 2 {
        panic!("Should be larger than 2.0")
    }
    120.0 * f / p as f32
}

/// Calculates the slip of a motor.
///
/// # Arguments
///
/// * `n_s` - The synchronous speed of the motor in RPM.
/// * `n_r` - The rotor speed of the motor in RPM.
///
/// # Returns
///
/// * `f32` - The slip of the motor.
///
/// # Formula
///
/// The formula used to calculate the slip \( S \) is:
/// $S = \frac{N_s - N_r}{N_s}$
///
pub fn motor_slip(n_s: f32, n_r: f32) -> f32 {
    (n_s - n_r) / n_s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn velocity_from_slip(slip:f32, frequency:f32, poles: u8)->f32{
        let synchronous_velocity = synchronous_speed(frequency,poles);
        - synchronous_velocity * (slip - 1.0)
    }
    fn calculate_motor_velocity(power_output:f32, line_frequency:f32, poles:u8, slip_function:fn(f32)->f32) -> f32 {
        let slip = slip_function(power_output);
        velocity_from_slip(slip,line_frequency,poles)
    }

    #[test]
    fn it_works() {
        let n_s = synchronous_speed(60.0, 4);

        let slip_f = |x: f32| -> f32 {
            0.25 * (1.0 - (-x / 100.0).exp()) + 0.1 * (-x / 10.0).exp()
        };

        let required_power = 50.0;
        let slip = slip_f(required_power);
        println!("slip: {}", slip_f(50.0));
        let n_r = -n_s * (slip - 1.0);
        println!("n_s: {}", n_s);
        println!("n_r: {n_r}");
        assert_eq!(n_r,calculate_motor_velocity(required_power,60.0,4,slip_f))
    }
}
