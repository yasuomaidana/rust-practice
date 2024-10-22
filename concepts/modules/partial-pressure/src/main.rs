const N_MASS: f64 = 14.00674;
const H_MASS: f64 = 1.00794;

const R: f64 = 0.0821; // L atm / K mol


/// Calculates the pressure of a gas given its volume in liters, moles, and temperature in kelvin.
///
/// # Parameters
/// - `volume`: The volume of the gas in liters.
/// - `moles`: The amount of substance of the gas in moles.
/// - `temperature`: The temperature of the gas in kelvin.
///
/// # Returns
/// The pressure of the gas in atm.
fn pressure(volume: f64, moles: f64, temperature: f64) -> f64 {
    (moles * R * temperature) / volume
}
fn main() {
    let n2_molar_mass = N_MASS * 2.0;
    let n2_mass = 113.0;

    let n2_moles = n2_mass / n2_molar_mass;
    println!("N2 moles: {:.2}", &n2_moles);
    // Reaction formula
    // N2 + 3H2 -> 2NH3
    // H2 moles required
    let h2_moles = n2_moles * 3.0;
    println!("H2 moles required: {:.2}", &h2_moles);

    let nh3_moles = n2_moles * 2.0;
    println!("NH3 moles: {:.2}", &nh3_moles);

    let initial_mass = n2_mass + (h2_moles * 2.0 * H_MASS);
    let final_mass = nh3_moles * (N_MASS + (3.0 * H_MASS));
    let mass_diff = final_mass - initial_mass;
    assert_eq!(mass_diff, 0.0);

    let volume = 22.0;
    let temperature = 333.0;
    let nh3_pressure = pressure(volume, nh3_moles, temperature);
    println!("NH3 pressure: {:.2}", &nh3_pressure);

}
