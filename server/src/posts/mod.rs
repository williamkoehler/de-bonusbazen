use chrono::{DateTime, Utc};
use futures_util::TryStreamExt;
use sqlx::{Executor, Row};
use tracing::*;

pub mod error;
pub mod model;

#[derive(Clone)]
pub struct PostManager {
    postgres: sqlx::Pool<sqlx::postgres::Postgres>,
}

impl PostManager {
    pub async fn new(postgres: sqlx::Pool<sqlx::postgres::Postgres>) -> error::ResultNew<Self> {
        postgres.execute(sqlx::query("CREATE TABLE IF NOT EXISTS posts (id SERIAL PRIMARY KEY, visibility VARCHAR NOT NULL, created_at TIMESTAMPTZ NOT NULL, updated_at TIMESTAMPTZ NOT NULL, author INTEGER, title VARCHAR NOT NULL, metadata JSONB, body TEXT)"))
            .await
            .map_err(|err|{
                error!("failed to create posts table, due to database error: {}", err);
                error::ErrorNew::SqlxError { inner: err }
            })?;

        Ok(Self { postgres })
    }

    pub async fn posts(&self, show_hidden: bool) -> error::Result<Vec<model::Post>> {
        let mut posts = Vec::new();

        let mut query_builder = sqlx::QueryBuilder::new(
            "SELECT id, visibility, created_at, updated_at, author, title, SUBSTRING(body, 0, 600) as extract, metadata FROM posts WHERE ",
        );

        query_builder.push("visibility in (");
        {
            let mut separated = query_builder.separated(", ");

            if show_hidden {
                separated.push_bind(model::PostVisibility::Hidden);
                separated.push_bind(model::PostVisibility::Draft);
            }
            separated.push_bind(model::PostVisibility::Visible);
        }
        query_builder.push(")");

        let query = query_builder.build();

        let mut rows = query.fetch(&self.postgres);
        while let Some(row) = rows
            .try_next()
            .await
            .map_err(|err| error::Error::SqlxError { inner: err })?
        {
            let id: i32 = row
                .try_get("id")
                .map_err(|err| error::Error::SqlxError { inner: err })?;

            let visibility: model::PostVisibility = row
                .try_get("visibility")
                .map_err(|err| error::Error::SqlxError { inner: err })?;

            let created_at: DateTime<Utc> = row
                .try_get("created_at")
                .map_err(|err| error::Error::SqlxError { inner: err })?;

            let updated_at: DateTime<Utc> = row
                .try_get("updated_at")
                .map_err(|err| error::Error::SqlxError { inner: err })?;

            let author: i32 = row
                .try_get("author")
                .map_err(|err| error::Error::SqlxError { inner: err })?;

            let title: String = row
                .try_get("title")
                .map_err(|err| error::Error::SqlxError { inner: err })?;

            let extract: String = row
                .try_get("extract")
                .map_err(|err| error::Error::SqlxError { inner: err })?;

            let metadata: Option<sqlx::types::Json<model::PostMetadata>> = row
                .try_get("metadata")
                .map_err(|err| error::Error::SqlxError { inner: err })?;

            posts.push(model::Post {
                id,
                visibility,
                created_at,
                updated_at,
                author,
                title,
                body: Some(extract),
                metadata: metadata.map(|x| x.0),
            });
        }

        Ok(posts)
    }

    pub async fn post(&self, id: i32, show_hidden: bool) -> error::Result<model::Post> {
        let mut query_builder = sqlx::QueryBuilder::new(
            "SELECT id, visibility, created_at, updated_at, author, title, metadata, body FROM posts WHERE ",
        );

        query_builder.push("id = ");
        query_builder.push_bind(id);
        query_builder.push(" AND ");

        query_builder.push("visibility in (");
        {
            let mut separated = query_builder.separated(", ");

            if show_hidden {
                separated.push_bind(model::PostVisibility::Hidden);
            }
            separated.push_bind(model::PostVisibility::Visible);
        }
        query_builder.push(")");

        let query = query_builder.build();

        let row = query.fetch_one(&self.postgres).await.map_err(|err| {
            error!(
                id = id,
                "failed to get post, due to database error: {}", err
            );
            error::Error::SqlxError { inner: err }
        })?;

        let id: i32 = row
            .try_get("id")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let visibility: model::PostVisibility = row
            .try_get("visibility")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let created_at: DateTime<Utc> = row
            .try_get("created_at")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let updated_at: DateTime<Utc> = row
            .try_get("updated_at")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let author: i32 = row
            .try_get("author")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let title: String = row
            .try_get("title")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let metadata: Option<sqlx::types::Json<model::PostMetadata>> = row
            .try_get("metadata")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let body: String = row
            .try_get("body")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        Ok(model::Post {
            id,
            visibility,
            created_at,
            updated_at,
            author,
            title,
            body: Some(body),
            metadata: metadata.map(|x| x.0),
        })
    }

    pub async fn add_post(
        &self,
        visibility: model::PostVisibility,
        author: i32,
        title: &str,
        body: &str,
    ) -> error::Result<model::Post> {
        let created_at = chrono::Utc::now();
        let metadata = model::PostMetadata {};

        let row =
            sqlx::query("INSERT INTO posts (visibility, created_at, updated_at, author, title, metadata, body) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id")
                .bind(&visibility)
                .bind(&created_at)
                .bind(&created_at)
                .bind(author)
                .bind(title)
                .bind(sqlx::types::Json(&metadata))
                .bind(body)
                .fetch_one(&self.postgres)
                .await
                .map_err(|err| {
                    error!("failed to add post, due to database error: {}", err);
                    error::Error::SqlxError { inner: err }
                })?;

        let id: i32 = row
            .try_get("id")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        info!(id = id, author = author, title = title, "added post.");

        Ok(model::Post {
            id,
            visibility,
            created_at,
            updated_at: created_at,
            author,
            title: title.to_string(),
            body: None,
            metadata: Some(metadata),
        })
    }

    pub async fn update_post(
        &self,
        id: i32,
        visibility: Option<model::PostVisibility>,
        title: Option<&str>,
        body: Option<&str>,
    ) -> error::Result<()> {
        let mut query_builder = sqlx::QueryBuilder::new("UPDATE posts SET ");

        let mut separated = query_builder.separated(", ");

        if let Some(visibility) = visibility {
            separated.push("visibility = ");
            separated.push_bind_unseparated(visibility);
        }
        if let Some(title) = title {
            separated.push("title = ");
            separated.push_bind_unseparated(title);
        }
        // if let Some(metadata) = None {
        //     separated.push("metadata = ");
        //     separated.push_bind_unseparated(sqlx::types::Json(metadata));
        // }
        if let Some(body) = body {
            separated.push("body = ");
            separated.push_bind_unseparated(body);
        }

        if title.is_some() || body.is_some() {
            separated.push("updated_at = ");
            separated.push_bind_unseparated(chrono::Utc::now());
        }

        query_builder.push(" WHERE id = ");
        query_builder.push_bind(id);

        let query = query_builder.build();

        let result = self.postgres.execute(query).await.map_err(|err| {
            error!(
                id = id,
                "failed to update post, due to database error: {}", err
            );
            error::Error::SqlxError { inner: err }
        })?;

        if result.rows_affected() == 0 {
            error!(id = id, "failed to update post.");
            return Err(error::Error::OperationFailed {
                msg: "failed to update post.",
            });
        }

        info!(id = id, "updated post.");

        Ok(())
    }

    pub async fn remove_post(&self, id: i32) -> error::Result<()> {
        let result = self
            .postgres
            .execute(sqlx::query("DELETE FROM posts WHERE id = $1").bind(id))
            .await
            .map_err(|err| {
                error!(
                    id = id,
                    "failed to remove post, due to database error: {}", err
                );
                error::Error::SqlxError { inner: err }
            })?;

        if result.rows_affected() == 0 {
            error!(id = id, "failed to remove post.");
            return Err(error::Error::OperationFailed {
                msg: "failed to remove post.",
            });
        }

        info!(id = id, "removed post.");

        Ok(())
    }
}
