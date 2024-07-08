use crate::data::data_errors::DataError;
use crate::models::post::{CreatePost, Post, PostFilter, UpdatePost};

pub trait PostRepository: Send + Sync + 'static {
    fn create_post(&self, create_post: CreatePost) -> Result<Post, DataError>;
    fn get_post(&self, id: uuid::Uuid) -> Result<Post, DataError>;
    fn get_posts(&self, post_filter: PostFilter) -> Result<Vec<Post>, DataError>;
    fn update_post(&self, id: uuid::Uuid, update_post: UpdatePost) -> Result<Post, DataError>;
}
