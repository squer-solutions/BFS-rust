// The main function is the entry point of a Rust program
// It takes no arguments, and returns nothing by default
fn main() {
    // Creating and assigning a variable on the stack is done with the let keyword
    // We don't have to specify a type as Rust uses type inference similar but not as
    // powerful as the Hindley–Milner type system (used in Haskell)

    // Rust uses a best effort approach to infer the type of variables, by default integers are i32
    // and floats are f64
    let x = 1;

    // We can also specify the type of a variable and not initialize it
    let y: i32;

    // We can't use y here because it is not initialized
    // let z = y + 1; // This will not compile

    // And assign it later
    y = 2;

    // But we cannot reassign it
    // y = 3; // This will not compile

    // Numeric operations are done with the usual operators
    // We can declare a variable as mutable with the mut keyword
    let mut z = x + y;

    // This is a macro. You can tell because it ends with an exclamation mark
    println!("Hello, world! I just calculated x + y and it is: {}", z); // Will print 3

    // Because we used mut we can reassign z
    z = 4;
    println!("I reassigned z and it now stores: {}", z); // Will print 4

    // This creates a new variable x that shadows the previous one
    // Logically the old x is still in scope, but lexically it impossible to access it
    let x = 22;
    println!("The value of the new x is: {}", x); // Will print 22

    // We can create arbitrary scopes
    {
        // We can access values from the enclosing scope
        println!("The value of x in the new scope is: {}", x); // Will print 22

        // We can also shadow the value of x
        let x = 3;
        println!("The new value of x in the new scope is: {}", x); // Will print 3
    }

    // The x from the inner scope exists neither logically nor lexically in the outer scope
    println!("The value of x outside the scope is still: {}", x); // Will print 22

    // Scopes can be used as expressions
    let scope_result = {
        // Static strings from our code are of type &'static str
        // We can convert them to a String with the to_string() method
        // The difference between a &str and a String is that the former is a reference to a string
        // and the latter is a heap-allocated string that can be mutated
        let a = "Hello";
        let b = "World";
        // Notice how we don't need the return keyword, the last expression in a block is the return
        // If we added a semicolon to the end of the line below, the block would return the unit type ()
        // This would cause the program to not compile because the unit type cannot be printed
        a.to_string() + " " + b
    };

    println!("The result of the scope is: {}", scope_result); // Will print "HelloWorld"

    // For collections, we have a few options
    // Arrays are fixed size
    // They are of the form [T; N] where T is the type and N is the size
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    let first = array[1]; // Accessing an element of an array is done with square brackets

    // This will panic if the index is out of bounds
    // A panic is a runtime error that will stop the program
    // array[7];

    // If we want a dynamic collection we can use a Vec
    // Vecs are heap allocated and growable
    let mut vec = vec![1, 2, 3];
    vec.push(1);
    vec.push(2);

    // We can access elements of a Vec with square brackets
    vec[0] = 3;

    // The contents of the Vec are now [3, 2, 3, 1, 2]

    // The following functions are from different modules, these are declared using the mod keyword
    // A module is a file or a directory with an `mod.rs` file
    structs::structured_fun();
    enums::enumerated_fun();
    traits::trait_fun();
    generics::generic_fun();

    // One last thing, as we have seen, error handling is usually done with the Result type
    // But sometimes an error is so catastrophic that it is better to panic
    // A panic will unwind the stack and print an error message
    // Unlike an exception in other languages, a panic is not meant to be caught (there are ways, but the are not recommended)
    // In general the philosophy is as follows:
    // - Use Result for recoverable errors
    // - Use panic for unrecoverable errors, so things like out of memory, or some fundamental invariant being broken
    // panic!("This is a panic message");

    // The Result types has some convenience methods, like unwrap, which will return the value if it is Ok, and panic if it is Err
    // This is useful for prototyping, but in production code it is better to handle the error
    // The exception is calling unwrap on things that YOU know will never be an error

    // There is no point in handling an error that will never happen
    let int = "42".parse::<i32>().unwrap();
}

// This is how we declare a module
mod enums;
mod generics;
mod structs;
mod traits;
