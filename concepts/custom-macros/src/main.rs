#[macro_use] mod macros;
#[macro_use] mod printer;
// mod printer;

create_function!(foo);
create_function!(bar);

fn main() {
    say_hello!();
    foo();
    bar();
    print_result!(1u32 + 1);
    
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });

    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        let _ = 2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);

    let a = create_printer!("hp-z800");
    let b = create_printer!("hp-z800", "2A");
    let c = create_printer!("hp-z800", "2A", "WiFi");

    println!("Printer A: {} - {} - {}", a.name, a.model, a.connection_type);
    println!("Printer B: {} - {} - {}", b.name, b.model, b.connection_type);
    println!("Printer C: {} - {} - {}", c.name, c.model, c.connection_type);
}
