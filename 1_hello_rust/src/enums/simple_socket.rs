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

    pub fn send_data(&self, _data: &[u8]) -> Result<(), &str> {
        // Based on the state of the socket we can send data
        // Notice that we have a return type of Result but
        // no return keyword. This is because match is an expression
        // All arms are also of the same type
        match self {
            Socket::Connected { ip, port } => {
                // Imagine sending data here
                println!("Sending data to {}:{}", ip, port);
                Ok(())
            }
            // We use a wildcard pattern to match all other states
            _ => Err("Cannot send data, socket is not connected"),
        }

        // This could also have been achieved with an if let and an else branch
    }
}
