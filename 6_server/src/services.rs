use std::sync::Arc;

use crate::data::repositories::post_repository::PostRepository;
use crate::data::repositories::user_repository::UserRepository;

pub trait ServiceProvider: UserRepositoryProvider + PostRepositoryProvider {}

pub trait UserRepositoryProvider: Clone + Send + Sync + 'static {
    fn user_repository(&self) -> Arc<dyn UserRepository>;
}

pub trait PostRepositoryProvider: Clone + Send + Sync + 'static {
    fn post_repository(&self) -> Arc<dyn PostRepository>;
}
