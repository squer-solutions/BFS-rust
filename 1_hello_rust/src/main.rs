// The main function is the entry point of a Rust program
// It takes no arguments, and returns nothing by default
fn main() {
    // Creating and assigning a variable on the stack is done with the let keyword
    // We don't have to specify a type as Rust uses type inference similar but not as
    // powerful as the Hindley–Milner type system (used in Haskell)

    // Rust uses a best effort approach to infer the type of a variable, by default integers are i32
    // and floats are f64
    let x = 1;

    // We can also specify the type of a variable and not initialize it
    let y: i32;

    // And assign it later
    y = 2;

    // Numeric operations are done with the usual operators
    let z = x + y;

    // This is a macro, not a function Macros come in two flavors: declarative and procedural
    // For now we don't really care how or why they work
    println!("Hello, world! I just calculated x + y and it is: {}", z);

    // x = 2; // This will not compile because x is immutable
    let x = 2; // This creates a new variable x that shadows the previous one
    println!("The value of the new x is: {}", x); // Will print 2

    // We can create arbitrary scopes
    {
        let x = 3;
        println!("The value of x in the new scope is: {}", x); // Will print 3
    }

    println!("The value of x outside the scope is still: {}", x); // Will print 2

    // Scopes are evaluated
    let scope_result = {
        // Static strings from our code are of type &'static str
        // We can convert them to a String with the to_string() method
        // The difference between a &str and a String is that the former is a reference to a string
        // and the latter is a heap-allocated string
        let a = "Hello";
        let b = "World";
        // Notice how we don't need the return keyword, the last expression in a block is the return
        // If we added a semicolon to the end of the line below, the block would return the unit type ()
        // This would cause the program to not compile because the unit type cannot be printed
        a.to_string() + " " + b
    };

    println!("The result of the scope is: {}", scope_result); // Will print "HelloWorld"

    // Lets now do two things at once:
    // Call a function from another module
    let result = structs::my_function();

    println!("The result of the function is: {}", result); // Will print "Hello from a function in a module"
}

// This is how we import a module
// A module is either a file or a directory with an `mod.rs` file
mod structs;
