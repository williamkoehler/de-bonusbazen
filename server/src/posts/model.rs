use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::Type, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[sqlx(type_name = "VARCHAR")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "snake_case")]
pub enum PostVisibility {
    Visible,
    Draft,
    Hidden,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct Post {
    pub id: i32,
    pub visibility: PostVisibility,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author: i32,
    pub title: String,
    pub body: Option<String>,
    pub metadata: Option<PostMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostMetadata {}