use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "VARCHAR")]
#[sqlx(rename_all = "lowercase")]
pub enum RawRights {
    Unauthenticated,
    Normal,
    Member,
    Maintainer,
    Admin,
}

pub struct RawUser {
    pub id: i32,
    pub name: String,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub rights: RawRights,
    pub has_profile_picture: bool,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "VARCHAR")]
#[sqlx(rename_all = "lowercase")]
pub enum RawPostVisibility {
    Visible,
    Draft,
    Hidden,
}

pub struct RawPost {
    pub id: i32,
    pub visibility: RawPostVisibility,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author: i32,
    pub title: String,
    pub extract: Option<String>,
    pub metadata: Option<RawPostMetadata>,
}

#[derive(Serialize, Deserialize)]
pub struct RawPostMetadata {}