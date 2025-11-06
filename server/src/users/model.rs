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