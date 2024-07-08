use axum::{middleware, Router};

use crate::server::middlewares::tracing::tracing_middleware;
use crate::server::routers::post_router::post_router;
use crate::server::routers::user_router::user_router;
use crate::server::state::AppState;

pub fn define_app(state: AppState) -> Router {
    Router::new()
        .nest("/api",
              Router::new()
                  .nest("/user", user_router(state.clone()))
                  .nest("/post", post_router(state)))
        .layer(middleware::from_fn(tracing_middleware))
}
