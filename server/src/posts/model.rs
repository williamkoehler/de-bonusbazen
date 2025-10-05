use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct Post {
    id: i32,
    visibility: PostVisibility,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    title: String,
    author: i32,
    metadata: Option<PostMetadata>,
    body: Option<String>,
}

impl Post {
    pub fn id(&self) -> &i32 {
        &self.id
    }

    pub fn visibility(&self) -> &PostVisibility {
        &self.visibility
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    pub fn author(&self) -> i32 {
        self.author
    }

    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn metadata(&self) -> Option<&PostMetadata> {
        self.metadata.as_ref()
    }

    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }
}

impl From<crate::database::model::RawPost> for Post {
    fn from(raw_post: crate::database::model::RawPost) -> Self {
        Self {
            id: raw_post.id,
            visibility: raw_post.visibility.into(),
            created_at: raw_post.created_at,
            updated_at: raw_post.updated_at,
            author: raw_post.author,
            title: raw_post.title,
            metadata: raw_post.metadata.map(|x| x.into()),
            body: raw_post.extract,
        }
    }
}

impl From<(crate::database::model::RawPost, String)> for Post {
    fn from(value: (crate::database::model::RawPost, String)) -> Self {
        let (raw_post, body) = value;
        Self {
            id: raw_post.id,
            visibility: raw_post.visibility.into(),
            created_at: raw_post.created_at,
            updated_at: raw_post.updated_at,
            author: raw_post.author,
            title: raw_post.title,
            metadata: raw_post.metadata.map(|x| x.into()),
            body: Some(body),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PostVisibility {
    Hidden,
    Draft,
    Visible,
}

impl From<crate::database::model::RawPostVisibility> for PostVisibility {
    fn from(value: crate::database::model::RawPostVisibility) -> Self {
        match value {
            crate::database::model::RawPostVisibility::Visible => Self::Visible,
            crate::database::model::RawPostVisibility::Draft => Self::Draft,
            crate::database::model::RawPostVisibility::Hidden => Self::Hidden,
        }
    }
}

impl Into<crate::database::model::RawPostVisibility> for PostVisibility {
    fn into(self) -> crate::database::model::RawPostVisibility {
        match self {
            Self::Visible => crate::database::model::RawPostVisibility::Visible,
            Self::Draft => crate::database::model::RawPostVisibility::Draft,
            Self::Hidden => crate::database::model::RawPostVisibility::Hidden,
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct PostMetadata {}

impl PostMetadata {}

impl From<crate::database::model::RawPostMetadata> for PostMetadata {
    fn from(_value: crate::database::model::RawPostMetadata) -> Self {
        Self {}
    }
}
