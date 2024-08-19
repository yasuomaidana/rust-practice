pub struct Person{
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) age: u8
}
impl Person {
    pub(crate) fn new(first_name: String, last_name: String, age: u8) -> Person {
        Person {
            first_name,
            last_name,
            age
        }
    }
}