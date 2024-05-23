// Welcome to the structs module!

// For the purposes of this example we will declare a couple of type aliases
// We do this to not have to instantiate the actual types (that would needlessly complicate the example)
type UdpSocket = ();
type IpAddr = String;
type Port = u16;

// Rust has 4 types for grouping values: tuples, unions, enums, and structs
// We won't be looking at unions as they are rather rare and not useful in a "safe" context

// Let's start off with the most conventional grouping type: structs
// They are similar to classes in other languages, they group data together
// In memory they are stored as a contiguous block of memory

// The first type of grouping is an enum
// Enums are a way to group related values together
// They can be used similar to enums in other languages where
// it as a type can only ever have one of a set of values
#[derive(Debug)]
pub enum MaritalStatus {
    Single,
    Married,
    Divorced,
    Widowed,
}

// Here we declare a struct with two fields: name and age
// Fields are private to a module by default, we can make them public by adding the pub keyword
// Rust has no concept of inheritance
#[derive(Debug)]
pub struct Person {
    pub name: String,
    age: u8,
    marital_status: MaritalStatus,
}

// The real power of enums in Rust comes from the ability to attach data to each variant
// This allows us to guarantee that certain values are only ever present when a certain variant is used
// allowing the compiler to catch many bugs at compile time
enum SocketStatus {
    // We can mix and match different types of data
    // We can have a variant with no data
    Init,
    // We can have a variant with a single unnamed piece of data
    Open(UdpSocket),
    // We can have a variant with multiple named pieces of data (much like a struct)
    Error { error: String, code: u32 },
    Closed,
}

// The final grouping type is a tuple
// It is rarely usually declared as a return type of function or used as a generic parameter
// We can however create a type alias
// In future use they Point type will mean a tuple of two i32 values
type EndPoint = (IpAddr, Port);

// Ok, now let's define some basic behavior for a struct
// Simple as can be
pub struct ManagedUpdSocket {
    socket: UdpSocket,
    status: SocketStatus,
}

// Now lets define some behavior for our struct
impl ManagedUpdSocket {
    // Rust does not have any concept of constructors
    // Instead we define a function that returns an instance of the struct
    // It is idiomatic to name this function new and have it return Self
    // In the context of a struct Self is the type we are implementing on
    pub fn new() -> Self {
        Self {
            socket: (),
            status: SocketStatus::Init,
        }
    }

    // We can also define methods that take a reference to self
    // This is similar to the pointer in other languages
    // As we want to change the state of the struct we take a mutable reference
    pub fn open(&mut self) {
        self.status = SocketStatus::Open(());
    }

    // Alternatively we can take ownership of self
    // This is useful when we want to consume the struct
    // The concept of ownership will be covered in the next chapter
    pub fn close(mut self) {
        // We can't use self after this line
        self.status = SocketStatus::Closed;
    }

    // Finally we take advantage of our state
    pub fn send(&mut self, person: &Person, end_point: &EndPoint) -> Result<(), String> {
        // We can match on the status of the socket
        // Ignore the & for now
        // The match statement is similar to a switch statement in other languages
        // It allows us to match on the value of a variable and execute code based on that
        // All branches need to be exhaustive, meaning we need to cover all possible values
        // If we want to ignore a value we can use the _ pattern, as this will match anything
        match &self.status {
            SocketStatus::Init => Err("Socket is not open yet".to_string()),
            SocketStatus::Open(socket) => {
                // This is the only scope that we can use the socket in

                // And for the sake of the example we will check if the person is too young
                // and have the socket go into an error state if they are
                if person.age < 18 {
                    self.status = SocketStatus::Error {
                        error: "Person was too young to send".to_string(),
                        code: 1,
                    };
                    // Here we need an explicit return. We could omit it,
                    // but then we would need to put the Ok(()) into an else block and omit the semicolon
                    // That would make the return type of the "if expression" a Result<(), String>
                    return Err("Person is too young to send".to_string());
                }

                // Let utilize the tuple we got
                let (ip, port) = end_point;

                // But we don't have a real socket so we will just return Ok
                println!(
                    "Sending person: {:?} using socket {:?} to {}:{}",
                    person, socket, ip, port
                );
                Ok(())
            }
            SocketStatus::Error { error, code } => Err(format!(
                "Socket is in an error state: {} with code: {}",
                error, code
            )),
            SocketStatus::Closed => Err("Socket is closed".to_string()),
        }
    }
}

// Now lets put this all to use
pub fn my_function() -> String {
    // Let's create a new Person
    let person = Person {
        name: "John Doe".to_string(),
        age: 30,
        marital_status: MaritalStatus::Single,
    };

    // Let's create a new socket
    let mut socket = ManagedUpdSocket::new();

    // And create a new endpoint
    let end_point: EndPoint = ("localhost".to_string(), 8080);

    // Try to send the person
    match socket.send(&person, &end_point) {
        Ok(_) => println!("Person was sent successfully"),
        Err(e) => println!("Failed to send person: {}", e),
    }

    // Let's open the socket
    socket.open();

    // Try to send the person again
    match socket.send(&person, &end_point) {
        Ok(_) => println!("Person was sent successfully"),
        Err(e) => println!("Failed to send person: {}", e),
    }

    // Now lets make the person too young
    let young_person = Person {
        name: "Jack Doe".to_string(),
        age: 17,
        marital_status: MaritalStatus::Divorced,
    };

    // Try to send the person again
    match socket.send(&young_person, &end_point) {
        Ok(_) => println!("Person was sent successfully"),
        Err(e) => println!("Failed to send person: {}", e),
    }

    // Even if we try to send an appropriate person the socket is in an error state
    match socket.send(&person, &end_point) {
        Ok(_) => println!("Person was sent successfully"),
        Err(e) => println!("Failed to send person: {}", e),
    }

    // Let's close the socket
    socket.close();

    // Well, that's it for now. Let's return something nice to the main function

    "Hello from a function in a module".to_string()
}
