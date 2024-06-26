use std::sync::Arc;

use crate::data::repositories::post_repository::PostRepository;
use crate::data::repositories::user_repository::UserRepository;

#[derive(Clone)]
pub struct AppState<UR: UserRepository, PR: PostRepository> {
    pub user_repository: Arc<UR>,
    pub posts_repository: Arc<PR>,
}

impl<UR: UserRepository, PR: PostRepository> AppState<UR, PR> {
    pub fn new(ur: UR, pr: PR) -> Self {
        AppState {
            user_repository: Arc::new(ur),
            posts_repository: Arc::new(pr),
        }
    }
}

pub trait UserRepositoryProvider: Clone + Send + Sync + 'static {
    fn user_repository(&self) -> Arc<impl UserRepository>;
}

impl<UR: UserRepository, PR: PostRepository> UserRepositoryProvider for AppState<UR, PR> {
    fn user_repository(&self) -> Arc<impl UserRepository> {
        self.user_repository.clone()
    }
}

pub trait PostRepositoryProvider: Clone + Send + Sync + 'static {
    fn post_repository(&self) -> Arc<impl PostRepository>;
}

impl<UR: UserRepository, PR: PostRepository> PostRepositoryProvider for AppState<UR, PR> {
    fn post_repository(&self) -> Arc<impl PostRepository> {
        self.posts_repository.clone()
    }
}