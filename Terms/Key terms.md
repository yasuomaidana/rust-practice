# Key terms

* Function: A block of code designed to carry out a specified task. In Rust, it's a crucial part of the language as Rust is almost like a functional programming language.

* Unit Function: A function that doesn't return any value but does some work when called.

* Return Value: The result given by a function upon completion. It can be explicitly defined or implicitly returned as unit type in case of unit functions.

* Borrowing: Concept in Rust which ensures efficient memory usage by safely lending values to other parts of the code without taking ownership away from their original scopes.

* Panic: A special call syntax in Rust used to stop all execution in a program; it's not commonly used in production code but can be helpful during development or for certain error handling scenarios.

* Control Flow: The order in which code is executed based on conditions and loops. In Rust, control flow includes if, else, match, and looping constructs like for or while.

* Enumerator (Enum): A data type representing a set of values where each value represents a distinct case. An example in the transcript is the Option\<T> enum with cases Some(T) and None.

* Move: In Rust, move occurs when ownership of a variable is transferred from one scope to another without any borrowing mechanism being used. This results in the original variable becoming invalid.

* Copy: A special trait in Rust which allows values of certain types (e.g., integers and booleans) to be copied instead of moved or borrowed when assigned or passed as arguments.

* Vector: A dynamic array data structure provided by Rust's standard library, used for storing a variable number of elements efficiently. It can grow or shrink in size during runtime.

* **Shadowing**: In Rust, the ability to redefine a variable with a new value within the same scope hiding the original definition but not invalidating it. This is useful when changing types locally without affecting other parts of the code.
