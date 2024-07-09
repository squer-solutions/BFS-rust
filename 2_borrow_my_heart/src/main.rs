#![allow(warnings)]

// Here we will be looking into the ownership model of Rust
// It is this model that allows Rust to work as a low level language
// without the need for a garbage collector

// The Rust book describes these three rules for ownership:
// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

// Now lets see how this works in practice
fn main() {
    // We can create a string on the heap with the String type
    let s = String::from("hello");

    // We can also create a string literal with the &str type
    let mut s2 = "hello";

    // If we mutate a String, we will change its contents
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // We cannot reassign a string literal, but we can reassign the reference
    let mut s2 = "hello";
    s2 = "world";
    println!("{}", s2); // Will print "world"


    // Currently all these values are owned by the main function
    // When the function goes out of scope, the values will be dropped
    // We can also create a new scope
    {
        let s = String::from("hello");
        // s is valid here
        println!("{}", s);
        // s is dropped here
    }

    // We can also reassign ownership
    let s1 = String::from("hello");
    let s2 = s1;

    // This will not compile because s1 has been moved
    // println!("{}", s1);

    // s1 hasn't been dropped, but its value has been moved to s2
    // Therefore you can no longer use s1

    // If you need a duplicate of a value, you can use the clone method
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    // Both s1 and s2 are valid here

    // The function takes_ownership takes ownership of the string it is passed. Meaning that
    // the value is moved into it.

    // Both s1 and s2 can be taken here because they are separate values
    takes_ownership(s1);
    takes_ownership(s2);

    // Again, this is not valid because s1 and s2 have been moved
    // println!("s1 = {}, s2 = {}", s1, s2);

    // Ownership means that you hold the value and are responsible for cleaning it up
    // If you wish to allow a function to use a value without taking ownership, you can pass a reference
    let s = String::from("hello");
    // Calculate length takes a reference to a string
    // References are denoted by the & symbol
    let len = calculate_length(&s);
    println!("The length of '{}' is {}", s, len);

    // We can hand out as many references as we want

    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);


    // References can only be read from, not written to
    // References must always be valid. The compiler will not allow you to create a dangling reference

    // To make sure of this Rust utilized the concept of lifetimes, which we will look at later

    // For now, we will look at mutable references

    // Mutable references are denoted by &mut. Note the difference between a mut &T and a &mut T.
    // The former is a mutable reference to an immutable value, the latter is an immutable reference to a mutable value
    // mut &mut T is a mutable reference to a mutable value

    // They work in a similar way to immutable references, but they allow you to change the value

    let mut s = String::from("hello");
    mutate(&mut s);
    println!("{}", s);

    // The difference being, a mutable reference mus the be ONLY reference to the value

    let mut s = String::from("hello");
    let r1 = &mut s;
    // This will not compile because we can't have two mutable references
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // We can't have a mutable reference and an immutable reference
    // let r1 = &s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // The println! is essential in the above code because otherwise the compiler will realize
    // that r1 is not used after r2 is created and allow the compilation


    // As mentioned earlier, Rust uses lifetimes to ensure that references are always valid
    // We will look at what this means in this module
    lifetimes::lifetimes();

    // The two types of references are the most common way to handle ownership in Rust
    // But sometimes they are not enough. This is why smart pointers exist
    smart_pointers::smart_pointers();
}

// Takes ownership of a string
fn takes_ownership(s: String) {
    println!("I am now the owner of: {}", s);
    // s is dropped here
}

// Takes a reference to a string
fn calculate_length(s: &String) -> usize {
    s.len()
    // The value of s is not dropped here
}

// Takes a mutable reference to a string
fn mutate(s: &mut String) {
    s.push_str(", world!");
    // The value of s is not dropped here
}

mod lifetimes;
mod smart_pointers;
