use axum::{middleware, Router};

use crate::data::repositories::post_repository::PostRepository;
use crate::data::repositories::user_repository::UserRepository;
use crate::server::middlewares::tracing::tracing_middleware;
use crate::server::routers::post_router::post_router;
use crate::server::routers::user_router::user_router;
use crate::server::state::AppState;

pub mod state;
pub mod handlers;
pub mod routers;
pub mod error_handlers;
pub mod middlewares;

pub fn define_app<UR: UserRepository, PR: PostRepository>(state: AppState<UR, PR>) -> Router {
    Router::new()
        .nest("/api",
              Router::new()
                  .nest("/user", user_router(state.clone()))
                  .nest("/post", post_router(state)))
        .layer(middleware::from_fn(tracing_middleware))
}
