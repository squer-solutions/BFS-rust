use crate::data::data_errors::DataError;
use crate::models::user::{CreateUser, User};

pub trait UserRepository: Send + Sync + 'static {
    fn create_user(&self, create_user: CreateUser) -> Result<User, DataError>;
    fn get_user(&self, id: uuid::Uuid) -> Result<User, DataError>;
}
