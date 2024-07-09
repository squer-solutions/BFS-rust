use std::net::{IpAddr, SocketAddr};

use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "HOST", default = "0.0.0.0")]
    host: IpAddr,
    #[envconfig(from = "PORT", default = "8080")]
    port: u16,
    #[envconfig(nested = true)]
    pub db: DbConfig,
}

#[derive(Envconfig)]
pub struct DbConfig {
    #[envconfig(from = "DATABASE_URL")]
    pub url: String,
}

impl Config {
    pub fn to_socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }
}