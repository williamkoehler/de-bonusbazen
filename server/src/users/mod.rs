use tracing::*;

use crate::databases::postgres::{PostgresDb, model};

pub mod error;

pub mod helper;

#[derive(Clone)]
pub struct UserManager {
    database: PostgresDb,
}

impl UserManager {
    pub async fn new(database: PostgresDb) -> error::ResultNew<Self> {
        if database
            .user_count()
            .await
            .map_err(|err| error::ErrorNew::Error { inner: err.into() })?
            == 0
        {
            let hash = helper::generate_hash("admin");

            database
                .add_user(
                    "admin",
                    Some("Admin"),
                    None,
                    &hash,
                    crate::databases::postgres::model::UserRights::Admin,
                )
                .await
                .map_err(|err| error::ErrorNew::Error { inner: err.into() })?;
        }

        Ok(Self { database })
    }

    pub async fn users(
        &self,
        include_members: bool,
        include_normal: bool,
    ) -> error::Result<Vec<model::User>> {
        let raw_users = self
            .database
            .users(include_members, include_normal, false)
            .await
            .map_err(|err| {
                error!("failed to get users: {}", err);
                error::Error::DatabaseError { inner: err }
            })?;

        Ok(raw_users
            .into_iter()
            .map(|raw_user| raw_user.into())
            .collect())
    }

    pub async fn user(&self, id: i32) -> error::Result<model::User> {
        let raw_user = self.database.user(id).await.map_err(|err| {
            error!(id = id, "failed to get user: {}", err);
            error::Error::DatabaseError { inner: err }
        })?;

        Ok(raw_user.into())
    }

    pub async fn user_by_name(&self, name: &str) -> error::Result<model::User> {
        let raw_user = self
            .database
            .user_by_name(name)
            .await
            .map_err(|err| error::Error::DatabaseError { inner: err })?;

        Ok(raw_user.into())
    }

    pub async fn user_and_hash_by_name(&self, name: &str) -> error::Result<(model::User, String)> {
        let (raw_user, hash) = self
            .database
            .user_and_hash_by_name(name)
            .await
            .map_err(|err| error::Error::DatabaseError { inner: err })?;

        Ok((raw_user.into(), hash))
    }

    pub async fn user_profile_picture(&self, id: i32) -> error::Result<Option<Vec<u8>>> {
        self.database
            .user_profile_picture(id)
            .await
            .map_err(|err| error::Error::DatabaseError { inner: err })
    }

    pub async fn add_user(
        &self,
        name: &str,
        nickname: Option<&str>,
        email: Option<&str>,
        password: &str,
        rights: model::UserRights,
    ) -> error::ResultAddUser<model::User> {
        // Check name, nickname, email and password content
        {
            // Check name
            {
                let name_regex = regex::Regex::new(r"^(?:\p{L}|[_])+$").map_err(|_| {
                    error::ErrorAddUser::InternalError {
                        msg: "invalid name regex".to_string(),
                    }
                })?;
                if !name_regex.is_match(name) {
                    return Err(error::ErrorAddUser::InvalidName {
                        name: name.to_string(),
                    });
                }
            }

            // Check nickname
            if let Some(nickname) = nickname {
                let nickname_regex = regex::Regex::new(r"^(?:\p{L}|[ _])+$").map_err(|_| {
                    error::ErrorAddUser::InternalError {
                        msg: "invalid nickname regex".to_string(),
                    }
                })?;
                if !nickname_regex.is_match(nickname) {
                    return Err(error::ErrorAddUser::InvalidNickname {
                        nickname: nickname.to_string(),
                    });
                }
            }

            // Check email
            if let Some(email) = email {
                let email_regex = regex::Regex::new(
                    r"^(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*)@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])$")
                    .map_err(|_| error::ErrorAddUser::InternalError { msg: "invalid email regex".to_string() })?;
                if !email_regex.is_match(email) {
                    return Err(error::ErrorAddUser::InvalidEMail {
                        email: email.to_string(),
                    });
                }
            }

            // Check password
            if password.len() < 8 {
                return Err(error::ErrorAddUser::InvalidPassword);
            }
        }

        let hash = helper::generate_hash(password);

        let raw_user = self
            .database
            .add_user(name, nickname, email, &hash, rights.into())
            .await
            .map_err(|err| match err {
                crate::databases::postgres::error::ErrorAddUser::UniqueNameConstraintViolation => {
                    return error::ErrorAddUser::NameIsTaken {
                        name: name.to_string(),
                    };
                }
                crate::databases::postgres::error::ErrorAddUser::UniqueEMailConstraintViolation => {
                    return error::ErrorAddUser::EMailIsTaken {
                        email: email.unwrap_or_default().to_string(),
                    };
                }
                err => {
                    return error::ErrorAddUser::DatabaseError { inner: err };
                }
            })?;

        Ok(raw_user.into())
    }

    pub async fn update_user(
        &self,
        id: i32,
        name: Option<&str>,
        nickname: Option<Option<&str>>,
        email: Option<Option<&str>>,
        password: Option<&str>,
        rights: Option<model::UserRights>,
        profile_picture: Option<&[u8]>,
    ) -> error::Result<()> {
        let profile_picture = if let Some(profile_picture) = profile_picture {
            // Load image from memory
            let image = image::load_from_memory(profile_picture).map_err(|err| {
                warn!("profile picture is invalid: {}", err);
                error::Error::InvalidData {
                    msg: "invalid image data",
                }
            })?;

            // Resize if too big (example: max 800x800)
            let (w, h) = image::GenericImageView::dimensions(&image);
            let max_dimension = 800;
            let image = if w > max_dimension || h > max_dimension {
                image.resize(
                    max_dimension,
                    max_dimension,
                    image::imageops::FilterType::Lanczos3,
                )
            } else {
                image
            };

            // Convert image to jpeg
            let mut buffer = Vec::new();
            let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buffer, 75);
            encoder.encode_image(&image).map_err(|err| {
                error!("failed to convert profile picture to jpeg: {}", err);
                error::Error::InvalidData {
                    msg: "failed to encode image data",
                }
            })?;

            Some(buffer)
        } else {
            None
        };

        let hash = if let Some(password) = password {
            Some(helper::generate_hash(password))
        } else {
            None
        };

        self.database
            .update_user(
                id,
                name,
                nickname,
                email,
                hash.as_deref(),
                rights.map(|x| x.into()),
                profile_picture.as_deref(),
            )
            .await
            .map_err(|err| error::Error::DatabaseError { inner: err })?;

        Ok(())
    }

    pub async fn remove_user(&self, id: i32) -> error::Result<()> {
        self.database
            .remove_user(id)
            .await
            .map_err(|err| error::Error::DatabaseError { inner: err })?;

        Ok(())
    }
}
