
use crate::person::Person;
use crate::family::Family;

pub fn generate_persons()->Vec<Person>{
    let anya = Person::new(String::from("Anya C"), String::from("Smith"), 32);
    let beth = Person::new(String::from("Beth"), String::from("Smith"), 30);
    let carl = Person::new(String::from("Carl"), String::from("Smith"), 28);
    let dave = Person::new(String::from("Dave"), String::from("Jones"), 32);
    let emily = Person::new(String::from("Emily"), String::from("Jones"), 30);
    let frank = Person::new(String::from("Frank"), String::from("Jones"), 28);
    let george = Person::new(String::from("George"), String::from("Brown"), 40);
    let helen = Person::new(String::from("Helen"), String::from("Brown"), 38);
    let ian = Person::new(String::from("Ian"), String::from("White"), 45);
    let jessica = Person::new(String::from("Jessica"), String::from("White"), 43);
    let kevin = Person::new(String::from("Kevin"), String::from("Black"), 50);
    vec![anya, beth, carl, dave, emily, frank, george, helen, ian, jessica, kevin]
}
pub fn generate_families<'a>() -> Vec<Family<'a>> {
    let family1 = Family::new(String::from("Smith"));
    let family2 = Family::new(String::from("Jones"));
    let family3 = Family::new(String::from("Brown"));
    let family4 = Family::new(String::from("White"));
    let family5 = Family::new(String::from("Black"));
    vec![family1, family2, family3, family4, family5]
}
pub fn fill_families<'a>(people: &'a mut Vec<Person>, families: &mut Vec<Family<'a>>) {
    for person in people.iter() {
        match person.last_name.as_str() {
            "Smith" => families[0].add_member(person),
            "Jones" => families[1].add_member(person),
            "Brown" => families[2].add_member(person),
            "White" => families[3].add_member(person),
            "Black" => families[4].add_member(person),
            _ => println!("No family found")
        }
    }
    families[1].add_member(&people[0]);
}
// pub fn generate_families(person: Vec<Person>) -> Vec<Family<'static>> {
//     let mut family1 = Family::new(String::from("Smith"));
//     let anya = Person::new(String::from("Anya C"), String::from("Smith"), 32);
//     let beth = Person::new(String::from("Beth"), String::from("Smith"), 30);
//     let carl = Person::new(String::from("Carl"), String::from("Smith"), 28);
//     family1.add_member(&anya);
//     family1.add_member(&beth);
//     family1.add_member(&carl);
//
//     let mut family2 = Family::new(String::from("Jones"));
//     let dave = Person::new(String::from("Dave"), String::from("Jones"), 32);
//     let emily = Person::new(String::from("Emily"), String::from("Jones"), 30);
//     let frank = Person::new(String::from("Frank"), String::from("Jones"), 28);
//     family2.add_member(&dave);
//     family2.add_member(&emily);
//     family2.add_member(&frank);
//     family2.add_member(&anya);
//
//     let mut family3 = Family::new(String::from("Brown"));
//     let george = Person::new(String::from("George"), String::from("Brown"), 40);
//     let helen = Person::new(String::from("Helen"), String::from("Brown"), 38);
//     family3.add_member(&george);
//     family3.add_member(&helen);
//
//     let mut family4 = Family::new(String::from("White"));
//     let ian = Person::new(String::from("Ian"), String::from("White"), 45);
//     let jessica = Person::new(String::from("Jessica"), String::from("White"), 43);
//     family4.add_member(&ian);
//     family4.add_member(&jessica);
//
//     let mut family5 = Family::new(String::from("Black"));
//     let kevin = Person::new(String::from("Kevin"), String::from("Black"), 50);
//     family5.add_member(&kevin);
//
//
//     vec![family1, family2, family3, family4, family5]
// }