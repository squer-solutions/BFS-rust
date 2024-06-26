use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "HOST", default = "0.0.0.0")]
    pub host: String,
    #[envconfig(from = "PORT", default = "8080")]
    pub port: u16,
    #[envconfig(nested = true)]
    pub db: DbConfig,
}

#[derive(Envconfig)]
pub struct DbConfig {
    #[envconfig(from = "DATABASE_URL")]
    pub url: String,
}