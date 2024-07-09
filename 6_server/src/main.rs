use envconfig::Envconfig;
use tracing::log::info;

use crate::config::Config;
use crate::data::db::postgres::Postgres;
use crate::server::app::define_app;

pub mod data;
pub mod server;
pub mod models;
pub mod config;
pub mod services;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let _ = dotenvy::dotenv();

    let config = Config::init_from_env().expect("Failed to load config");

    let postgres = Postgres::new(&config.db).expect("Failed to connect to Postgres");

    let app = define_app(postgres.into());

    let addr = config.to_socket_addr();
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind to address");

    info!("Server listening on: {}", addr);

    axum::serve(listener, app).await.expect("Failed to start server");
}
