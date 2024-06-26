pub fn structured_fun() {
    // Here we will be looking at how structs work in Rust
    // Structs allow you to create complex data structures
    // They are similar to classes in other languages

    // We can define a struct
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // We can create an instance of the struct
    let user1 = User {
        email: "office@squer.io".to_string(),
        username: "squer".to_string(),
        active: true,
        sign_in_count: 1,
    };

    // We can access the fields of the struct
    println!("The username of the user is: {}", user1.username);

    // We can destructure the struct
    let User {
        email,
        username,
        active,
        sign_in_count,
    } = user1;

    // Now we can use the variables
    println!("The email of the user is: {}", email);

    // If we define a struct
    struct Point {
        x: i32,
        y: i32,
    }

    // And then create an instance of the struct
    let point = Point { x: 1, y: 2 };

    // We can use the update syntax to create a new instance of the struct
    // This keeps all old values and only updates the ones we specify
    // This is useful when we have a struct with many fields
    let new_point = Point { x: 3, ..point };

    // We can implement an associated function for the struct
    impl Point {
        // This is an associated function
        // New is a common name for a function that creates a new instance of a struct
        fn new(x: i32, y: i32) -> Self {
            Self { x, y }
        }
    }

    // We can now create a new instance of the struct like this
    let point = Point::new(1, 2);

    // We can also implement methods for the struct
    impl Point {
        // This is a method
        fn distance(&self, other: &Point) -> f64 {
            let x = (self.x - other.x).pow(2) as f64;
            let y = (self.y - other.y).pow(2) as f64;
            (x + y).sqrt()
        }
    }

    // We can now call the method like this
    let point1 = Point::new(1, 1);
    let point2 = Point::new(4, 5);
    let distance = point1.distance(&point2);
    println!("The distance between the two points is: {}", distance);

    // We can create a tuple struct
    // A tuple struct is a struct with unnamed fields
    struct Color(u8, u8, u8);

    // We can create an instance of the tuple struct like this
    let black = Color(0, 0, 0);

    // We can access the fields of the tuple struct like this
    println!("The first value of the color is: {}", black.0);

    // Rust has first class support for tuples
    // We can create a tuple like this
    let tuple = (1, 2, 3);

    // We can destructure the tuple like this
    let (a, b, c) = tuple;

    // Tuples are useful when we want to return multiple values from a function
    // To be frank, returning a Point would have made more sense here
    fn find_middle(point1: Point, point2: Point) -> (i32, i32) {
        ((point1.x + point2.x) / 2, (point1.y + point2.y) / 2)
    }
}
