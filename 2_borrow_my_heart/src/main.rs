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

    // We can also create a string on the stack with the &str type
    let s2 = "hello";

    // We can't do this because the string is on the stack
    // s2.push_str(", world!");

    // We can do this because the string is on the heap
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // We can also clone the string
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    // Both s1 and s2 can be taken here because they are
    // different now
    takes_ownership(s1);
    takes_ownership(s2);

    // We can also pass the string to a function
    let s = String::from("hello");
    takes_ownership(s);
    // This will not compile because s has been moved
    // println!("{}", s);

    // We can also pass a reference to the string
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}", s, len);

    // We can also pass a mutable reference to the string
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    // We can only have one mutable reference to a value
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

    // We can have multiple immutable references
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);

    // Now we will take a look at smart pointers
    smart_pointers::smart_pointers();

    // And finally lifetimes
    lifetimes::lifetimes();
}

fn takes_ownership(s: String) {
    println!("I am now the owner of: {}", s);
    // Conceptually the string s is dropped here
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}

mod lifetimes;
mod smart_pointers;
