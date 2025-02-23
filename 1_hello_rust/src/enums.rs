use std::net::IpAddr;
use std::str::FromStr;

pub fn enumerated_fun() {
    // Here we will look at one of the more powerful features of Rust: enums

    // Enums are a way to define a type by enumerating its possible values
    // Where a struct is a product type, an enum is a sum type

    // Let's say we want to define an IP address type
    enum IpAddrKind {
        V4,
        V6,
    }

    // We can now create instances of the enum
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // We can match on the enum
    fn route(ip_kind: IpAddrKind) {
        // The match keyword is used for pattern matching
        // It is similar to switch in other languages but more flexible
        // It can destructure complex data types and is used in many places in Rust
        match ip_kind {
            // The match arms must always be exhaustive
            IpAddrKind::V4 => println!("Routing to IPv4"),
            IpAddrKind::V6 => println!("Routing to IPv6"),
        }
    }

    route(four);
    route(six);

    // Let's look at a neat match example
    for x in 0..10 {
        // Here we construct a tuple and match on it
        match (x % 3, x % 5) {
            // Match happens top to bottom
            (0, 0) => println!("FizzBuzz"),
            // A _ is a wildcard
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{}", x),
        }
    }

    // So far enum and match are similar to other languages
    // But Rust adds a secret sauce: enum variants can have data

    // Let's say we want to store IP addresses in one type
    // In other languages you'd have to either use a class hierarchy or
    // some is_a function to determine the type
    // In Rust we can use enums with data to store the version and the address
    // together. This makes it impossible to have an IPv4 type with an IPv6 address

    // This is exactly how the IpAddr type in the standard library is defined

    let four = IpAddr::from_str("127.0.0.1").unwrap();
    let six = IpAddr::from_str("::1").unwrap();

    // We can match on the enum
    fn route2(ip: IpAddr) {
        match ip {
            // We can destructure the enum variant, and get the data
            // But ONLY if it is the correct variant
            IpAddr::V4(addr) => println!("Routing to IPv4 address: {}", addr),
            IpAddr::V6(addr) => println!("Routing to IPv6 address: {}", addr),
        }
    }

    route2(four);
    route2(six);

    // Some of the most commonly used enums are Option and Result
    // Option is used when a value can be absent
    // Result is used when an operation can fail

    // Let's look at Option
    // Option<T> is an enum with two variants: Some(T) and None where T is a generic type
    // It is used to represent the presence or absence of a value
    let some_numbers = vec![Some(1), None, Some(3)];

    for some_number in some_numbers {
        // We can match on Option
        match some_number {
            Some(n) => println!("The number is: {}", n),
            None => println!("There is no number"),
        }
    }

    // Result<T,E> is an enum with two variants: Ok(T) and Err(E)
    // It is used to return a value or an error
    // Generally used as a return type for functions that can fail

    // Both option and result have a lot of convenience methods
    // Like unwrap, expect, map, and many more

    // We can define our own error type for use with Result
    #[derive(Debug)]
    enum MyError {
        Not1(i32),
        MathError,
    }

    // We define the return type as Result<T, E> where T and E are generic types
    // T is the type of the value we want to return
    // E is the type of the error
    // We will look at generics later
    fn check_if_one(n: i32) -> Result<i32, MyError> {
        if n == 1 {
            Ok(n)
        } else {
            Err(MyError::Not1(n))
        }
    }

    match check_if_one(1) {
        Ok(_) => println!("The number is 1"),
        Err(e) => println!("The number is not 1: {:?}", e),
    }

    match check_if_one(2) {
        Ok(_) => println!("The number is 1"),
        // We can match in a match
        Err(e) => match e {
            MyError::Not1(actual) => println!("The number is not 1, it is: {}", actual),
            _ => println!("Some other error"),
        },
    }

    // Enums can hold more than just tuples
    // They can hold structs, enums, and even other enums
    // All these different types of data can be mixed and matched
    // This makes them very powerful
    // To show this off you will find a VERY simple socket implementation in simpleSocket.rs

    // Create a new socket
    let mut socket = simple_socket::Socket::new();

    let ip = IpAddr::from_str("127.0.0.1").unwrap();
    socket.connect(ip, 443);

    // We can now send data
    match socket.send_data(b"Hello world") {
        Ok(_) => println!("Data sent successfully"),
        Err(e) => println!("Error sending data: {}", e),
    }

    // We can also disconnect
    socket.disconnect("User requested".to_string());

    // And try to send data again
    match socket.send_data(b"Hello world") {
        Ok(_) => println!("Data sent successfully"),
        Err(e) => println!("Error sending data: {}", e),
    }
}

mod simple_socket;
