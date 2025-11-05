use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::Type, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[sqlx(type_name = "VARCHAR")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "snake_case")]
pub enum UserRights {
    Unauthenticated,
    Normal,
    Member,
    Maintainer,
    Admin,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub rights: UserRights,
    pub has_profile_picture: bool,
}

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
    pub extract: Option<String>,
    pub metadata: Option<PostMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostMetadata {}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct AhProduct {
    pub id: u64,
    pub name: String,
    pub image: Option<String>,
    pub bonus: bool,
    pub price: Option<f64>,
    pub price_before_bonus: Option<f64>,
}
