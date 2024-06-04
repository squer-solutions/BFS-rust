pub fn generic_fun() {
    // Rust has generics
    // They work similar to how generics work in other languages

    // Lets say we have a function that returns the first element of a list
    // We can write it like this
    fn first_element<T>(list: Vec<T>) -> T {
        for elem in list {
            return elem;
        }
        panic!("The list is empty");
    }

    // We can now call this function with a list of integers
    let list = vec![1, 2, 3];
    // We don't need to specify the type of the list, Rust will infer it
    let first = first_element(list);
    println!("The first element of the list is: {}", first);
    // And with a list of strings
    let list = vec!["hello", "world"];
    let first = first_element(list);
    println!("The first element of the list is: {}", first);

    // What happens here is that the compiler creates a new version of the function for each type
    // Meaning, this is not dynamic dispatch, but static dispatch

    // We can also have multiple types
    fn first_two_elements<T, U>(list: Vec<T>, list2: Vec<U>) -> (T, U) {
        // We can even pass the generic parameters to other functions
        (first_element(list), first_element(list2))
    }

    let list = vec![1, 2, 3];
    let list2 = vec!["hello", "world"];
    let (first, second) = first_two_elements(list, list2);
    println!("The first element of the first list is: {}", first);
    println!("The first element of the second list is: {}", second);

    // We can also have generic structs
    struct Container<T> {
        child: T,
    }

    // Finally, we can set trait bounds for generics
    fn is_greater_than<T: PartialOrd>(left: T, right: T) -> bool {
        // We can use the functions of the trait, like cmp
        left > right
    }
}
