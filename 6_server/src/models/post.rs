use nutype::nutype;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: uuid::Uuid,
    pub title: Title,
    pub body: String,
    pub published: bool,
    pub author: uuid::Uuid,
}

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 100, len_char_min = 3),
    derive(Debug, PartialEq, Clone, Serialize, Deserialize, Display)
)]
pub struct Title(String);

#[derive(Debug, Deserialize)]
pub struct CreatePost {
    pub title: Title,
    pub body: String,
    pub author: uuid::Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePost {
    pub title: Option<Title>,
    pub body: Option<String>,
    pub published: Option<bool>,
    pub author: Option<uuid::Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct PostFilter {
    pub title: Option<String>,
    pub published: Option<bool>,
    pub author: Option<uuid::Uuid>,
}
