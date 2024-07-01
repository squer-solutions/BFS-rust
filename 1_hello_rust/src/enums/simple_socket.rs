use std::net::IpAddr;

// We can define our socket state as an enum
pub enum Socket {
    Uninitialized,
    Connected { ip: IpAddr, port: u16 },
    Disconnected(String),
}

// Let's implement some methods on the enum
impl Socket {
    pub fn new() -> Self {
        Socket::Uninitialized
    }

    // Don't be bothered by all the mut and &, we will cover that later
    pub fn connect(&mut self, ip: IpAddr, port: u16) {
        // We can use if let to match on a single variant
        // This is the alternative to match
        if let Socket::Uninitialized = self {
            *self = Socket::Connected { ip, port };
        }
    }

    pub fn disconnect(&mut self, reason: String) {
        // You can see the familiar .. syntax here.
        // It is used to indicated that we don't care about any of the fields
        // The curly braces can also be omitted
        if let Socket::Connected { .. } = self {
            *self = Socket::Disconnected(reason);
        }
    }

    // Data has the type &[u8], which is a slice of bytes.
    // We don't want to specify a specific type, because we want to be able to send any kind of data
    // Slice is a reference to a sequence of elements in a contiguous memory block,
    // Slices can be constructed from different types, making our function more flexible
    pub fn send_data(&self, _data: &[u8]) -> Result<(), &str> {
        // Based on the state of the socket we can send data
        // Notice that we have a return type of Result but
        // no return keyword. This if is an expression, where all the arms return the same type
        if let Socket::Connected { ip, port } = self {
            // Imagine sending data here
            println!("Sending data to {}:{}", ip, port);
            Ok(())
        } else {
            Err("Cannot send data, socket is not connected")
        }
    }
}
