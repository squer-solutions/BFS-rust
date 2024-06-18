fn main() {
    // Rust has a very powerful iterator system
    // Iterators are what is called a zero cost abstraction

    // We can create an iterator from a range
    let range = 0..=3;
    // And then use it in a for loop
    // This consumes the iterator, meaning we can't use it again
    for i in range {
        println!("The value of i is: {}", i);
    }

    // No longer possible
    // range.nth(0);

    // We can also create an iterator from an array
    let array = [1, 2, 3, 4, 5];
    let iterator = array.iter();
    // This iterator takes ownership of all values in the array

    // Iterators have a lot of methods that can be chained
    let sum = iterator
        .map(|x| x * 2)
        .filter(|x| x % 2 == 0)
        .fold(0, |acc, x| acc + x);

    println!("The sum of the even numbers in the array is: {}", sum);

    // Iterators are lazy, meaning they only do work when asked
    let v = vec![1, 2, 3, 4, 5];
    let iter = v.iter().map(|x| x * x).map(|x| {
        // Side effect. Generally not recommended
        println!("The value in the iterator is: {}", x);
        x
    });

    // The println! macro is only called when we iterate over the iterator
    println!("The iterator is created");
    let sum: i32 = iter.sum();
    println!("The sum of the squares of the values is: {}", sum);

    // We can also create infinite iterators
    let infinite = std::iter::repeat(1);
    // We can use the take method to limit the number of values
    let sum: i32 = infinite.take(5).sum();
    println!("The sum of the first 5 ones is: {}", sum);

    // All of these functions take a closure as an argument
    // Closures are Rust's way of defining anonymous functions
    let c = |x| x + 1;
    let a = c(1);
    let b = c(2);

    // Closures can capture variables from the environment
    let c = |x| x + a;
    c(b);

    // However, this changed the type of the closure
    // Instead of being a Fn(i32) -> i32 it is now a FnOnce(i32) -> i32
    // The difference is that an Fn can be called multiple times, while an FnOnce can only be called once
    // This is because the closure captures values, making it impossible to uphold ownership rules otherwise

    // Lastly we have FnMut(). This is a closure that can be called multiple times, but can mutate captured variables
    let mut x = 1;
    let mut square_x = || x *= x;

    square_x();
    println!("The value of x is: {}", x);
}
