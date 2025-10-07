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

impl From<crate::databases::postgres::model::RawUser> for User {
    fn from(raw_user: crate::databases::postgres::model::RawUser) -> Self {
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

impl From<crate::databases::postgres::model::RawUserRights> for Rights {
    fn from(value: crate::databases::postgres::model::RawUserRights) -> Self {
        match value {
            crate::databases::postgres::model::RawUserRights::Unauthenticated => Self::Unauthenticated,
            crate::databases::postgres::model::RawUserRights::Normal => Self::Normal,
            crate::databases::postgres::model::RawUserRights::Member => Self::Member,
            crate::databases::postgres::model::RawUserRights::Maintainer => Self::Maintainer,
            crate::databases::postgres::model::RawUserRights::Admin => Self::Admin,
        }
    }
}

impl Into<crate::databases::postgres::model::RawUserRights> for Rights {
    fn into(self) -> crate::databases::postgres::model::RawUserRights {
        match self {
            Self::Unauthenticated => crate::databases::postgres::model::RawUserRights::Unauthenticated,
            Self::Normal => crate::databases::postgres::model::RawUserRights::Normal,
            Self::Member => crate::databases::postgres::model::RawUserRights::Member,
            Self::Maintainer => crate::databases::postgres::model::RawUserRights::Maintainer,
            Self::Admin => crate::databases::postgres::model::RawUserRights::Admin,
        }
    }
}