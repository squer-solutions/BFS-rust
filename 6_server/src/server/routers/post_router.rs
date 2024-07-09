use axum::Router;
use axum::routing::{get, post, put};

use crate::server::handlers::post_handlers::{create_post, get_all_posts, get_post, update_post};
use crate::services::PostRepositoryProvider;

pub fn post_router<T: PostRepositoryProvider>(state: T) -> Router {
    Router::new()
        .route("/", get(get_all_posts::<T>))
        .route("/:id", get(get_post::<T>))
        .route("/", post(create_post::<T>))
        .route("/:id", put(update_post::<T>)).with_state(state)
}