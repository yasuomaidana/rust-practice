use crate::real_animals::{Animal, Elephant, Human, Lion, LivingStatus};

mod real_animals;


fn main() {
    let mut lion = Lion::new("Simba".to_string());
    let elephant = Elephant::new("Dumbo".to_string());
    let human = Human::new("John".to_string());

    lion.set_status(LivingStatus{age: 10, is_alive: false });

    let living_things: Vec<&dyn Animal> = vec![&lion, &elephant, &human];

    for living_thing in living_things{
        let status = living_thing.get_status();
        let (age, is_alive) = (status.age, status.is_alive);
        println!("Name is: {} is {} years old and {}", living_thing.get_name(), age, if is_alive {"alive"} else {"dead"} );

    }

    let living_speakers: Vec<&dyn real_animals::LivingSpeaker> = vec![&lion, &elephant, &human];
    for speaker in living_speakers{
        speaker.make_sound();
    }

    println!("Instantiating by names");
    let status = LivingStatus{age: 0, is_alive: true};
    let name = "Rambo".to_string();
    let lion = Lion{status, name};
    println!("Lion name is: {}", lion.get_name());
}
