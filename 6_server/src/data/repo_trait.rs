use crate::data::repositories::post_repository::PostRepository;
use crate::data::repositories::user_repository::UserRepository;

pub trait DataRepository: UserRepository + PostRepository + Clone {}
