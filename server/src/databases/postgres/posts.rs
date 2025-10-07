use chrono::DateTime;
use chrono::Utc;
use futures_util::TryStreamExt;
use sqlx::{Executor, Row};

use crate::databases::postgres::model::*;

use super::error;
use super::model;

impl super::PostgresDb {
    pub async fn post_count(&self) -> error::Result<usize> {
        let row = sqlx::query("SELECT count(*) as count FROM posts WHERE visibility = 'visible'")
            .fetch_one(&self.pool)
            .await
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let count: i64 = row
            .try_get("count")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        Ok(count as usize)
    }

    pub async fn posts(
        &self,
        include_hidden: bool,
        include_draft: bool,
        include_visible: bool,
    ) -> error::Result<Vec<model::RawPost>> {
        let mut posts = Vec::new();

        let mut query_builder = sqlx::QueryBuilder::new(
            "SELECT id, created_at, updated_at, author, title, SUBSTRING(body, 0, 600) as extract, metadata FROM posts WHERE ",
        );

        query_builder.push("visibility in (");
        {
            let mut separated = query_builder.separated(", ");

            if include_hidden {
                separated.push_bind(RawPostVisibility::Hidden);
            }
            if include_draft {
                separated.push_bind(RawPostVisibility::Draft);
            }
            if include_visible {
                separated.push_bind(RawPostVisibility::Visible);
            }
        }
        query_builder.push(")");

        let query = query_builder.build();

        let mut rows = query.fetch(&self.pool);
        while let Some(row) = rows
            .try_next()
            .await
            .map_err(|err| error::Error::SqlxError { inner: err })?
        {
            let id: i32 = row
                .try_get("id")
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

            let metadata: Option<sqlx::types::Json<model::RawPostMetadata>> = row
                .try_get("metadata")
                .map_err(|err| error::Error::SqlxError { inner: err })?;

            posts.push(model::RawPost {
                id,
                visibility: model::RawPostVisibility::Visible,
                created_at,
                updated_at,
                author,
                title,
                extract: Some(extract),
                metadata: metadata.map(|x| x.0),
            });
        }

        Ok(posts)
    }

    pub async fn post_and_body(&self, id: i32) -> error::Result<(model::RawPost, String)> {
        let row = sqlx::query(
            "SELECT id, visibility, created_at, updated_at, author, title, metadata, body FROM posts WHERE id = $1",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| error::Error::SqlxError { inner: err })?;

        let id: i32 = row
            .try_get("id")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let created_at: DateTime<Utc> = row
            .try_get("created_at")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let updated_at: DateTime<Utc> = row
            .try_get("updated_at")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let visibility: model::RawPostVisibility = row
            .try_get("visibility")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let author: i32 = row
            .try_get("author")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let title: String = row
            .try_get("title")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let metadata: Option<sqlx::types::Json<model::RawPostMetadata>> =
            row.try_get("metadata")
                .map_err(|err| error::Error::SqlxError { inner: err })?;

        let body: String = row
            .try_get("body")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        Ok((
            model::RawPost {
                id,
                visibility,
                created_at,
                updated_at,
                author,
                title,
                extract: None,
                metadata: metadata.map(|x| x.0),
            },
            body,
        ))
    }

    pub async fn add_post(
        &self,
        visibility: model::RawPostVisibility,
        author: i32,
        title: &str,
        body: &str,
    ) -> error::Result<model::RawPost> {
        let created_at = chrono::Utc::now();
        let metadata = model::RawPostMetadata {};

        let row =
            sqlx::query("INSERT INTO posts (visibility, created_at, updated_at, author, title, metadata, body) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id")
                .bind(&visibility)
                .bind(&created_at)
                .bind(&created_at)
                .bind(author)
                .bind(title)
                .bind(sqlx::types::Json(&metadata))
                .bind(body)
                .fetch_one(&self.pool)
                .await
                .map_err(|err| error::Error::SqlxError { inner: err })?;

        let id: i32 = row
            .try_get("id")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        Ok(model::RawPost {
            id,
            visibility,
            created_at,
            updated_at: created_at,
            author,
            title: title.to_string(),
            extract: None,
            metadata: Some(metadata),
        })
    }

    pub async fn update_post(
        &self,
        id: i32,
        visibility: Option<model::RawPostVisibility>,
        title: Option<&str>,
        metadata: Option<&model::RawPostMetadata>,
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
        if let Some(metadata) = metadata {
            separated.push("metadata = ");
            separated.push_bind_unseparated(sqlx::types::Json(metadata));
        }
        if let Some(body) = body {
            separated.push("body = ");
            separated.push_bind_unseparated(hex::encode(body));
        }

        if title.is_some() || body.is_some() {
            separated.push("updated_at = ");
            separated.push_bind_unseparated(chrono::Utc::now());
        }

        query_builder.push(" WHERE id = ");
        query_builder.push_bind(id);

        let query = query_builder.build();

        let result = self
            .pool
            .execute(query)
            .await
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        if result.rows_affected() == 0 {
            return Err(error::Error::OperationFailed {
                msg: "failed to update user.",
            });
        }

        Ok(())
    }

    pub async fn remove_post(&self, id: i32) -> error::Result<()> {
        let result = self
            .pool
            .execute(sqlx::query("DELETE FROM posts WHERE id = $1").bind(id))
            .await
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        if result.rows_affected() == 0 {
            return Err(error::Error::OperationFailed {
                msg: "failed to remove post.",
            });
        }

        Ok(())
    }
}
