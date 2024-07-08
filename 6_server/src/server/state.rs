use std::sync::Arc;

use crate::data::repo_trait::DataRepository;
use crate::data::repositories::post_repository::PostRepository;
use crate::data::repositories::user_repository::UserRepository;
use crate::services::{PostRepositoryProvider, ServiceProvider, UserRepositoryProvider};

#[derive(Clone)]
pub struct AppState {
    pub user_repository: Arc<dyn UserRepository>,
    pub posts_repository: Arc<dyn PostRepository>,
}


impl<T: DataRepository> From<T> for AppState {
    fn from(repo: T) -> Self {
        AppState {
            user_repository: Arc::new(repo.clone()),
            posts_repository: Arc::new(repo),
        }
    }
}

impl ServiceProvider for AppState {}

impl UserRepositoryProvider for AppState {
    fn user_repository(&self) -> Arc<dyn UserRepository> {
        self.user_repository.clone()
    }
}

impl PostRepositoryProvider for AppState {
    fn post_repository(&self) -> Arc<dyn PostRepository> {
        self.posts_repository.clone()
    }
}