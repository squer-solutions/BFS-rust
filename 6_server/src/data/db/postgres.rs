use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::{ManageConnection, Pool};

use crate::config::DbConfig;
use crate::data::repo_trait::DataRepository;

#[derive(Clone)]
pub struct Postgres {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl Postgres {
    pub fn new(conf: DbConfig) -> Result<Self, anyhow::Error> {
        let manager = ConnectionManager::<PgConnection>::new(conf.url);
        manager.connect()?;
        let pool = Pool::builder().build(manager)?;

        Ok(Postgres { pool })
    }
}

impl DataRepository for Postgres {}
