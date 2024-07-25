fn example_2(){
    let foo = 69;
    let mut r;
    {
        let bar = 42;
        r = &bar;
        println!("r: {}", r);
        println!("r: {}", *r);
    }
    r = &foo;
    println!("r: {}", r);

}

// Both s1 and s2 are references which have the same lifetime 'a
fn longest<'a>(s1:&'a str, s2:&'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    example_2();
    let s1 = String::from("abcd");
    let s2 = "xyz";
    let result = longest(&s1, s2);
    println!("The longest string is {}", result);
}
