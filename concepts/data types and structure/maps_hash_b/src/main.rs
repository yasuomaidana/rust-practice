use crate::families::{fill_families, generate_families, generate_persons};

mod person;
mod family;
mod families;

fn main() {
    let mut people = generate_persons();
    let mut families = generate_families();

    fill_families(&mut people, &mut families);
    for family in &families {
        println!("Family: {}", family.name);
        for member in &family.members {
            println!("\t {} {} age {}", member.first_name, member.last_name, member.age);
        }
    }


    println!("Hello, world!");
}
