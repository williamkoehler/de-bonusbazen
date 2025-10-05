use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct User {
    id: i32,
    name: String,
    nickname: Option<String>,
    email: Option<String>,
    rights: Rights,
    has_profile_picture: bool,
}

impl From<crate::database::model::RawUser> for User {
    fn from(raw_user: crate::database::model::RawUser) -> Self {
        Self {
            id: raw_user.id,
            name: raw_user.name,
            nickname: raw_user.nickname,
            email: raw_user.email,
            rights: raw_user.rights.into(),
            has_profile_picture: raw_user.has_profile_picture,
        }
    }
}

impl User {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn nickname(&self) -> Option<&str> {
        self.nickname.as_deref()
    }

    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }
    
    pub fn rights(&self) -> Rights {
        self.rights
    }

    pub fn has_profile_picture(&self) -> bool {
        self.has_profile_picture
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Rights {
    Unauthenticated,
    Normal,
    Member,
    Maintainer,
    Admin,
}

impl From<crate::database::model::RawRights> for Rights {
    fn from(value: crate::database::model::RawRights) -> Self {
        match value {
            crate::database::model::RawRights::Unauthenticated => Self::Unauthenticated,
            crate::database::model::RawRights::Normal => Self::Normal,
            crate::database::model::RawRights::Member => Self::Member,
            crate::database::model::RawRights::Maintainer => Self::Maintainer,
            crate::database::model::RawRights::Admin => Self::Admin,
        }
    }
}

impl Into<crate::database::model::RawRights> for Rights {
    fn into(self) -> crate::database::model::RawRights {
        match self {
            Self::Unauthenticated => crate::database::model::RawRights::Unauthenticated,
            Self::Normal => crate::database::model::RawRights::Normal,
            Self::Member => crate::database::model::RawRights::Member,
            Self::Maintainer => crate::database::model::RawRights::Maintainer,
            Self::Admin => crate::database::model::RawRights::Admin,
        }
    }
}