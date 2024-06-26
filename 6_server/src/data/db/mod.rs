use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use r2d2::ManageConnection;
use crate::config::DbConfig;

pub mod users_db;
pub mod posts_db;
mod schema;
mod db_error;

#[derive(Clone)]
pub struct Postgres {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Postgres {
    pub fn new(conf: DbConfig) -> Result<Self, anyhow::Error> {
        let manager = ConnectionManager::<PgConnection>::new(conf.url);
        manager.connect()?;
        let pool = Pool::builder().build(manager)?;

        Ok(Postgres { pool })
    }
}
