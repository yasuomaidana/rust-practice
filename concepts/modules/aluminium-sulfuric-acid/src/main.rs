const ALUMINIUM_MOLAR_MASS: f64 = 26.98;
//const SULFURIC_ACID_MOLAR_MASS: f64 = 98.08;
const SULFUR_MOLAR_MASS: f64 = 32.06;
const OXYGEN_MOLAR_MASS: f64 = 16.0;
const HYDROGEN_MOLAR_MASS: f64 = 1.0;


fn main() {
    const SULFURIC_ACID_MOLAR_MASS: f64 = HYDROGEN_MOLAR_MASS * 2.0
        + SULFUR_MOLAR_MASS + OXYGEN_MOLAR_MASS * 4.0;

    const ALUMINUM_SULFATE_MOLAR_MASS: f64 = ALUMINIUM_MOLAR_MASS * 2.0
        + (SULFUR_MOLAR_MASS + OXYGEN_MOLAR_MASS * 4.0) * 3.0;

    println!("---------------Molar masses-----------------");
    println!("Sulfuric acid molar mass: {:.2}", &SULFURIC_ACID_MOLAR_MASS);
    println!("Aluminum sulfate molar mass: {:.2}", &ALUMINUM_SULFATE_MOLAR_MASS);

    let aluminum_mass = 150.0;
    let sulfuric_acid_mass = 70.0;

    let aluminum_moles = aluminum_mass / ALUMINIUM_MOLAR_MASS;
    let sulfuric_acid_moles = sulfuric_acid_mass / SULFURIC_ACID_MOLAR_MASS;

    println!("---------------Moles-----------------");
    println!("Aluminum moles: {:.2}", &aluminum_moles);
    println!("Sulfuric acid moles: {:.2}", &sulfuric_acid_moles);

    let original_aluminum_to_sulfuric_acid_ratio = 2.0/3.0;
    let aluminum_to_sulfuric_acid_ratio = aluminum_moles / sulfuric_acid_moles;


    println!("---------------Limiting reactant-----------------");
    println!("Base aluminum to sulfuric acid ratio: {:.2}", &original_aluminum_to_sulfuric_acid_ratio);
    if aluminum_to_sulfuric_acid_ratio < original_aluminum_to_sulfuric_acid_ratio {
        println!("Aluminum is the limiting reactant");
    } else {
        println!("Sulfuric acid is the limiting reactant");
    }

    println!("---------------Products-----------------");
    let hydrogen_moles = sulfuric_acid_moles;
    let hydrogen_mass = hydrogen_moles * HYDROGEN_MOLAR_MASS * 2.0;

    let aluminum_sulfate_moles = sulfuric_acid_moles * 1.0 / 3.0;
    let aluminum_sulfate_mass = aluminum_sulfate_moles * ALUMINUM_SULFATE_MOLAR_MASS;
    println!("Hydrogen mass: {:.2}", &hydrogen_mass);
    println!("Aluminum sulfate mass: {:.2}", &aluminum_sulfate_mass);


    println!("---------------Remaining reactant-----------------");
    let consumed_aluminum_moles = 2.0 * aluminum_sulfate_moles;
    let remaining_aluminum_moles = aluminum_moles - consumed_aluminum_moles;
    let remaining_aluminum_mass = remaining_aluminum_moles * ALUMINIUM_MOLAR_MASS;
    println!("Remaining aluminum mass: {:.2}", &remaining_aluminum_mass);

    println!("Reaction Summary");
    println!("Aluminum consumed: {:.2}", &aluminum_mass);
    println!("Aluminum remaining: {:.2}", &remaining_aluminum_mass);
    println!("Sulfuric acid consumed: {:.2}", &sulfuric_acid_moles * SULFURIC_ACID_MOLAR_MASS);
    println!("Aluminum sulfate produced: {:.2}", &aluminum_sulfate_mass);
    println!("Hydrogen produced: {:.2}", &hydrogen_mass);

    println!("Reactant mass {:.2}", &aluminum_mass + &sulfuric_acid_mass);
    println!("Product mass {:.2}", &aluminum_sulfate_mass + &hydrogen_mass + &remaining_aluminum_mass);



}
