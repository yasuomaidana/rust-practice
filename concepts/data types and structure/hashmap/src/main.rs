use std::collections::HashMap;

fn key_checker(map: &HashMap<String, String>, key: &str) ->() {
    if map.contains_key(key) {
        println!("{} is a valid key", key);
    } else {
        println!("{} is not a valid key", key);
    }
}
fn main() {
    let mut reviews:HashMap<String, String> = HashMap::new();
    reviews.insert("The Martian".to_string(), "Excellent".to_string());
    reviews.insert("Blue Mars".to_string(), "Good".to_string());
    reviews.insert("The Expanse".to_string(), "Great".to_string());

    // iterating over the hashmap
    print!("Iterating over the hashmap\n");
    for (key, value) in &reviews {
        println!("{}: {}", key, value);
    }

    // Iterating over the keys
    print!("Iterating over the keys\n");
    for key in reviews.keys() {
        println!("{}", key);
    }

    // Accessing to a value
    print!("Accessing to a value\n");
    let key = "The Martian";
    match reviews.get(key) {
        Some(value) => println!("The review of {} is: {}", key, value),
        None => println!("{} is not a valid key", key),
    }
    
    println!("Valid {:?}",reviews.get(key));
    println!("Non Valid{:?}",reviews.get("key"));

    // Checking if key exists
    print!("Checking if key exists\n");
    let non_valid_key = "The Expanse";
    key_checker(&reviews, non_valid_key);
    let valid_key = "The Martian";
    key_checker(&reviews, valid_key);

    // Removing a key
    print!("Removing a key\n");
    reviews.remove("The Martian");
    key_checker(&reviews, valid_key);

    // Clearing the hashmap
    print!("Clearing the hashmap\n");
    let mut map_size = reviews.len();

    println!("The size of the map is: {}", map_size);
    reviews.clear();
    map_size = reviews.len();

    println!("The size of the map is: {}", map_size);

}
