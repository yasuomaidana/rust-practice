use std::collections::{BTreeMap, HashMap};
use crate::families::{fill_families, generate_families, generate_persons};
use crate::family::Family;

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
    println!("\n--------------------{}-------------------------\n", "Sorted by family size");
    let sorted_family_map = BTreeMap::from_iter(families.iter().map(|family| (family, family.members.len())));
    for (family, count) in sorted_family_map {
        println!("Family: {} has {} members", family.name, count);
    }
    println!("\n--------------------{}-------------------------\n", "Hashmap");
    let family_map: HashMap<&Family, usize> = HashMap::from_iter(families.iter().map(|family| (family, family.members.len())));
    for (family, count) in family_map.iter() {
        println!("Family: {} has {} members", family.name, count);
    }

    println!("\n--------------------{}-------------------------\n", "Binary map");
    let family_b_map = BTreeMap::from_iter(family_map.iter());
    for (family, count) in family_b_map.iter() {
        println!("Family: {} has {} members", family.name, count);
    }
}
