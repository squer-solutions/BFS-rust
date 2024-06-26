use anyhow::Error;
use diesel::prelude::*;

use crate::data::db::Postgres;
use crate::data::db::schema::users;
use crate::data::db::schema::users::table;
use crate::data::data_errors::DataError;
use crate::data::repositories::user_repository::UserRepository;
use crate::models::user::{CreateUser, User, Username};

#[derive(Insertable, Queryable)]
#[diesel(table_name = users)]
struct DbUser {
    id: uuid::Uuid,
    username: String,
    email: String,
}

impl TryFrom<DbUser> for User {
    type Error = Error;

    fn try_from(value: DbUser) -> Result<Self, Self::Error> {
        Ok(User {
            id: value.id,
            username: Username::new(value.username)?,
            email: value.email.parse()?,
        })
    }
}

#[derive(Insertable)]
#[diesel(table_name = users)]
struct DbCreateUser {
    username: String,
    email: String,
}

impl From<CreateUser> for DbCreateUser {
    fn from(user: CreateUser) -> Self {
        DbCreateUser {
            username: user.name.to_string(),
            email: user.email.to_string(),
        }
    }
}

impl UserRepository for Postgres {
    fn create_user(&self, create_user: CreateUser) -> Result<User, DataError> {
        let conn = &mut self.pool.get()?;
        let user: DbUser = diesel::insert_into(table)
            .values(DbCreateUser::from(create_user))
            .get_result(conn)?;
        Ok(user.try_into()?)
    }

    fn get_user(&self, id: uuid::Uuid) -> Result<User, DataError> {
        let conn = &mut self.pool.get()?;
        let user: DbUser = table.find(id).get_result(conn)?;
        Ok(user.try_into()?)
    }
}