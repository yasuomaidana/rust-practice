use std::fs;

pub(crate) fn read_cars(csv: &str) -> Vec<String> {
    csv.split(',')
        .map(|s| s.trim().to_string())
        .collect()
}

pub fn show_cars(cars: Vec<String>){
    println!("Cars: ");
    for car in cars {
        println!("{}", car);
    }
}

pub fn read_from_file(file: &str) -> Vec<String> {
    read_cars(&*fs::read_to_string(file)
        .expect("Could not read file"))
}