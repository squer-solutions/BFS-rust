use axum::Router;
use axum::routing::{get, post};

use crate::server::handlers::user_handlers::{create_user, get_user};
use crate::services::UserRepositoryProvider;

pub fn user_router<T: UserRepositoryProvider>(state: T) -> Router {
    Router::new()
        .route("/", post(create_user::<T>))
        .route("/:id", get(get_user::<T>))
        .with_state(state)
}