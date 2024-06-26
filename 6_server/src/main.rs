use std::net::{IpAddr, SocketAddr};

use envconfig::Envconfig;
use tracing::log::info;
use crate::config::Config;
use crate::data::db::Postgres;
use crate::server::define_app;
use crate::server::state::AppState;

pub mod data;
pub mod server;
pub mod models;
pub mod config;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let config = Config::init_from_env().expect("Failed to load config");
    let addr = SocketAddr::new(IpAddr::V4(config.host.parse().expect("Bad IP Addr")), config.port);

    let postgres = Postgres::new(config.db).expect("Failed to connect to Postgres");

    let app = define_app(AppState::new(postgres.clone(), postgres.clone()));

    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind to address");

    info!("Server listening on: {}", addr.to_string());

    axum::serve(listener, app).await.expect("Failed to start server");
}
