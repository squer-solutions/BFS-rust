use anyhow::Error;
use diesel::prelude::*;
use uuid::Uuid;

use crate::data::data_errors::DataError;
use crate::data::db::postgres::Postgres;
use crate::data::db::schema::posts;
use crate::data::db::schema::posts::dsl::*;
use crate::data::repositories::post_repository::PostRepository;
use crate::models::post::{CreatePost, Post, PostFilter, Title, UpdatePost};

#[derive(Insertable, Queryable)]
#[diesel(table_name = posts)]
struct DbPost {
    id: Uuid,
    title: String,
    body: String,
    published: bool,
    author_id: Uuid,
}

impl TryFrom<DbPost> for Post {
    type Error = anyhow::Error;

    fn try_from(value: DbPost) -> Result<Self, Self::Error> {
        Ok(Post {
            id: value.id,
            title: Title::new(value.title)?,
            body: value.body,
            published: value.published,
            author: value.author_id,
        })
    }
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
struct DbCreatePost {
    title: String,
    body: String,
}

impl From<CreatePost> for DbCreatePost {
    fn from(post: CreatePost) -> Self {
        DbCreatePost {
            title: post.title.to_string(),
            body: post.body,
        }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = posts)]
struct DbUpdatePost {
    title: Option<String>,
    body: Option<String>,
    published: Option<bool>,
}

impl From<UpdatePost> for DbUpdatePost {
    fn from(post: UpdatePost) -> Self {
        DbUpdatePost {
            title: post.title.map(|t| t.to_string()),
            body: post.body,
            published: post.published,
        }
    }
}

impl PostRepository for Postgres {
    fn create_post(&self, create_post: CreatePost) -> Result<Post, DataError> {
        let conn = &mut self.pool.get()?;

        Ok(diesel::insert_into(posts)
            .values(DbCreatePost::from(create_post))
            .get_result::<DbPost>(conn)?.try_into()?)
    }

    fn get_post(&self, search_id: Uuid) -> Result<Post, DataError> {
        let conn = &mut self.pool.get()?;

        Ok(posts.find(search_id)
            .get_result::<DbPost>(conn)?.try_into()?)
    }

    fn get_posts(&self, post_filter: PostFilter) -> Result<Vec<Post>, DataError> {
        let conn = &mut self.pool.get()?;

        let mut query = posts.into_boxed();

        if let Some(q_title) = post_filter.title {
            query = query.filter(title.like(format!("{}%", q_title)));
        }

        if let Some(p) = post_filter.published {
            query = query.filter(published.eq(p));
        }

        if let Some(a) = post_filter.author {
            query = query.filter(author_id.eq(a));
        }

        query.get_results(conn)?
            .into_iter()
            .map(DbPost::try_into).collect::<Result<Vec<Post>, Error>>().map_err(|e| e.into())
    }

    fn update_post(&self, post_id: uuid::Uuid, update_post: UpdatePost) -> Result<Post, DataError> {
        let conn = &mut self.pool.get()?;

        diesel::update(posts).filter(id.eq(post_id))
            .set(DbUpdatePost::from(update_post))
            .get_result::<DbPost>(conn)?.try_into().map_err(|e: anyhow::Error| e.into())
    }
}