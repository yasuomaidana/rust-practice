const OXYGEN_MOLAR_MASS: f64 = 16.0;
const HYDROGEN_MOLAR_MASS: f64 = 1.0;
const CARBON_MOLAR_MASS: f64 = 12.0;

fn main() {
    // Reaction formula
    // ETHANE
    // 2C2H6 + 7O2 -> 4CO2 + 6H2O
    let ethane_molar_mass = (2.0 * CARBON_MOLAR_MASS) + (6.0 * HYDROGEN_MOLAR_MASS);
    let oxygen_molar_mass = OXYGEN_MOLAR_MASS * 2.0;
    let water_molar_mass = (2.0 * HYDROGEN_MOLAR_MASS) + OXYGEN_MOLAR_MASS;
    let carbon_dioxide_molar_mass = (2.0 * OXYGEN_MOLAR_MASS) + CARBON_MOLAR_MASS;

    println!("---------------Molar masses-----------------");
    println!("Ethane molar mass: {:.2}", &ethane_molar_mass);
    println!("Oxygen molar mass: {:.2}", &oxygen_molar_mass);
    println!("Water molar mass: {:.2}", &water_molar_mass);
    println!("Carbon dioxide molar mass: {:.2}", &carbon_dioxide_molar_mass);

    let ethane_mass = 80.0;
    let oxygen_mass = 130.0;

    let ethane_moles = ethane_mass / ethane_molar_mass;
    let oxygen_moles = oxygen_mass / oxygen_molar_mass;

    println!("---------------Moles-----------------");
    println!("Ethane moles: {:.2}", &ethane_moles);
    println!("Oxygen moles: {:.2}", &oxygen_moles);

    println!("---------------Limiting reactant-----------------");
    let original_ethane_oxygen_ratio = 2.0 / 7.0; // Ethane to oxygen ratio
    println!("Base ethane to oxygen ratio: {:.2}", &original_ethane_oxygen_ratio);
    let ethane_oxygen_ratio = ethane_moles / oxygen_moles;
    println!("Ethane to oxygen ratio: {:.2}", &ethane_oxygen_ratio);
    if ethane_oxygen_ratio < original_ethane_oxygen_ratio {
        println!("Ethane is the limiting reactant");
    } else {
        println!("Oxygen is the limiting reactant");
    }

    let water_moles = oxygen_moles * 6.0 / 7.0;
    let carbon_dioxide_moles = oxygen_moles * 4.0 / 7.0;

    println!("---------------Products-----------------");
    println!("Water moles: {:.2}", &water_moles);
    println!("Carbon dioxide moles: {:.2}", &carbon_dioxide_moles);


    let water_mass = water_moles * water_molar_mass;
    let carbon_dioxide_mass = carbon_dioxide_moles * carbon_dioxide_molar_mass;
    println!("Water mass: {:.2}", &water_mass);
    println!("Carbon dioxide mass: {:.2}", &carbon_dioxide_mass);

    let consumed_ethane_moles = 2.0/6.0 * water_moles;
    println!("Consumed ethane moles: {:.2}", &consumed_ethane_moles);
    let remaining_ethane_moles = ethane_moles - consumed_ethane_moles;
    println!("Remaining ethane moles: {:.2}", &remaining_ethane_moles);
    let remaining_ethane_mass = remaining_ethane_moles * ethane_molar_mass;
    println!("Remaining ethane mass: {:.2}", &remaining_ethane_mass);


    let total_mass = ethane_mass + oxygen_mass;
    println!("Total mass: {:.2}", &total_mass);
    let final_mass = water_mass + carbon_dioxide_mass;
    println!("Final mass: {:.2}", &final_mass);
    let mass_diff = total_mass - final_mass - remaining_ethane_mass;
    println!("Mass difference: {:.2}", &mass_diff);

    let efficiency_ratio = 0.95;
    let water_mass_produced = water_mass * efficiency_ratio;
    println!("---------------Efficiency-----------------");
    println!("Water mass produced: {:.2}", &water_mass_produced);











}
