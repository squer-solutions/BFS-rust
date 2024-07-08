use axum::extract::{Path, Query, State};
use axum::Json;
use axum::response::Result;

use crate::models::post::{CreatePost, Post, PostFilter, UpdatePost};
use crate::services::PostRepositoryProvider;

pub async fn create_post<S: PostRepositoryProvider>(
    State(state): State<S>,
    Json(body): Json<CreatePost>,
) -> Result<Json<Post>> {
    Ok(Json(state.post_repository().create_post(body)?))
}

pub async fn get_post<S: PostRepositoryProvider>(
    State(state): State<S>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<Post>> {
    Ok(Json(state.post_repository().get_post(id)?))
}

pub async fn get_all_posts<S: PostRepositoryProvider>(
    State(state): State<S>,
    Query(filter): Query<PostFilter>,
) -> Result<Json<Vec<Post>>> {
    Ok(Json(state.post_repository().get_posts(filter)?))
}

pub async fn update_post<S: PostRepositoryProvider>(
    State(state): State<S>,
    Path(id): Path<uuid::Uuid>,
    Json(body): Json<UpdatePost>,
) -> Result<Json<Post>> {
    Ok(Json(state.post_repository().update_post(id, body)?))
}