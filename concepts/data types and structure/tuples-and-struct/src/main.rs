use std::fmt;
use std::fmt::Formatter;

fn print_header(header: &str) {
    let total_len = 60;

    let text_len = header.len();
    let padding_len = (total_len - text_len)/2;
    let right_padding_len = total_len - text_len - padding_len;
    let left_padding = "=".repeat(padding_len);
    let right_padding = "=".repeat(right_padding_len);
    println!("{}{}{}{}", left_padding, header, right_padding, "\n");
}
fn print_footer() {
    let total_len = 60;
    let mut padding = String::from("=").repeat(total_len);
    padding.pop();
    padding.insert(0, '|');
    padding.pop();
    padding.insert(padding.len(), '|');
    println!("\n{}", padding);
}

impl fmt::Display for NamedStructPerson {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}, Age: {}, Hobbies: {:?}", self.name, self.age, self.hobbies)
    }
}

struct NamedStructPerson {
    name: String,
    age: i32,
    hobbies: [String; 3],
}

fn match_struct_example(person: NamedStructPerson) {

    match &person {
        NamedStructPerson {age , name, .. }  if age< &18 =>{
            println!("Age 12");
            println!("\tName: {name}");
            println!("\tHobbies: IDK2");
        }
        NamedStructPerson{name, .. } if name =="Alias" =>{
            println!("Okay keep your secrets");
        }
        _ => {
            println!("{}", person);
        }
    }
}
#[derive(Debug)]
struct TupleStruct(String, i32, [i32; 3]);

fn main() {
    print_header("Tuples part 1: Accessing tuple elements");
    let tuple = ("Hi", 1, [1,2,3]);
    println!("Tuple: {:?}", tuple);
    println!("Accessing tuple elements by indexing: {}, {}, {:?}", tuple.0, tuple.1, tuple.2);
    let (text, number, list) = tuple;
    println!("Accessing tuple elements by destructuring: {}, {}, {:?}", text, number, list);
    print_footer();
    let tuple_vector = vec![("Hi", 1), ("Hello", 2)];

    print!("Iterating over tuple vector");
    println!("Iterating over tuple vector");
    for (text, number) in tuple_vector.iter() {
        println!("\tText part: {text}, Numeric part: {number}");
    }

    print_header("Struct part 2: Named struct and tuple struct");

    print_header("Named tuple struct");
    let person = NamedStructPerson {
        name: String::from("John"),
        age: 30,
        hobbies: ["Reading".to_string(), "Swimming".to_string(), "Coding".to_string()],
    };

    println!("Person: {:}", person);
    print!("Accessing to named struct elements: ");

    println!("Accessing named struct elements by destructuring");
    let NamedStructPerson {name, age, hobbies} = &person;
    println!("Name: {}, Age: {}, Hobbies: {:?}", name, age, hobbies);

    println!("Accessing named struct elements by indexing");
    println!("Name: {}, Age: {}, Hobbies: {:?}", person.name, person.age, person.hobbies);

    println!("Iterating over named struct hobbies");
    for hobby in person.hobbies.iter() {
        println!("\tHobby: {}", hobby);
    }

    println!("Tuple Struct");
    let tuple_struct = TupleStruct(String::from("John"), 30, [1,2,3]);
    println!("Tuple Struct: {:?}", tuple_struct);
    println!("Accessing tuple struct elements by indexing: {}, {}, {:?}", tuple_struct.0, tuple_struct.1, tuple_struct.2);
    println!("Accessing tuple struct elements by destructuring");
    let TupleStruct(name, age, list) = tuple_struct;
    println!("Name: {}, Age: {}, List: {:?}", name, age, list);

    print_footer();

    print_header("Struct part 3: Matching over structs");

    match_struct_example(person);
    let young_person = NamedStructPerson {
        name: String::from("Hello "),
        age: 12,
        hobbies: ["Reading".to_string(), "Swimming".to_string(), "Coding".to_string()],
    };
    let anonymous = NamedStructPerson {
        name: String::from("Alias"),
        age: 30,
        hobbies: ["Reading".to_string(), "Swimming".to_string(), "Coding".to_string()],
    };

    match_struct_example(young_person);
    match_struct_example(anonymous);



    print_footer();


}
