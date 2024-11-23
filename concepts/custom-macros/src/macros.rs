macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

// This macro takes an identifier as an argument and creates a
// function with that name.
macro_rules! create_function {
    ($func_name:ident) => {
        // The function, when called, prints a message indicating the function's name.
        // The empty line you are selecting serves as a separator between these two macro
        // definitions for better readability.
        fn $func_name() {
            // stringify! This macro will yield an expression of type &'static str 
            // which is the stringification of all the tokens passed to the macro.
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

macro_rules! print_result {
    ($expression:expr) => {
        // The stringify! macro is used to convert the expression given as an argument into a string.
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}
