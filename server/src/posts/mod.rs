use tracing::*;

use crate::databases::postgres::{PostgresDb, model};

pub mod error;

#[derive(Clone)]
pub struct PostManager {
    database: PostgresDb,
}

impl PostManager {
    pub async fn new(database: PostgresDb) -> error::ResultNew<Self> {
        Ok(Self { database })
    }

    pub async fn posts(&self, show_hidden: bool) -> error::Result<Vec<model::Post>> {
        let raw_posts = self
            .database
            .posts(show_hidden, show_hidden, true)
            .await
            .map_err(|err| {
                error!("failed to get posts: {}", err);
                error::Error::DatabaseError { inner: err }
            })?;

        Ok(raw_posts
            .into_iter()
            .map(|raw_post| raw_post.into())
            .collect())
    }

    pub async fn post(&self, id: i32, show_hidden: bool) -> error::Result<model::Post> {
        let (post, _) = self
            .database
            .post_and_body(id, show_hidden, show_hidden, true)
            .await
            .map_err(|err| {
                error!(id = id, "failed to get post: {}", err);
                error::Error::DatabaseError { inner: err }
            })?;

        Ok(post)
    }

    pub async fn add_post(
        &self,
        visibility: model::PostVisibility,
        author: i32,
        title: &str,
        body: &str,
    ) -> error::Result<model::Post> {
        let raw_post = self
            .database
            .add_post(visibility.into(), author, title, body)
            .await
            .map_err(|err| {
                error!(
                    author = author,
                    title = title,
                    "failed to add post: {}",
                    err
                );
                error::Error::DatabaseError { inner: err }
            })?;

        info!(
            id = raw_post.id,
            author = raw_post.author,
            title = raw_post.title,
            "added post."
        );

        Ok(raw_post.into())
    }

    pub async fn update_post(
        &self,
        id: i32,
        visibility: Option<model::PostVisibility>,
        title: Option<&str>,
        body: Option<&str>,
    ) -> error::Result<()> {
        self.database
            .update_post(id, visibility.map(|v| v.into()), title, None, body)
            .await
            .map_err(|err| {
                error!(id = id, "failed to update post: {}", err);
                error::Error::DatabaseError { inner: err }
            })?;

        info!(id = id, "updated post.");

        Ok(())
    }

    pub async fn remove_post(&self, id: i32) -> error::Result<()> {
        self.database.remove_post(id).await.map_err(|err| {
            error!(id = id, "failed to remove post: {}", err);
            error::Error::DatabaseError { inner: err }
        })?;

        info!(id = id, "removed post.");

        Ok(())
    }
}
