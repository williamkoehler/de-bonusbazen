use chrono::{DateTime, Utc};
use futures_util::TryStreamExt;
use sqlx::{Executor, Row};
use tracing::*;

pub mod error;
pub mod model;

pub mod helper;

#[derive(Clone)]
pub struct UserManager {
    postgres: sqlx::Pool<sqlx::postgres::Postgres>,
    redis: redis::aio::MultiplexedConnection,
}

impl UserManager {
    #[inline(always)]
    fn cache_key_profile_picture(id: i32) -> String {
        format!("user:{}:profile_picture", id)
    }

    #[inline(always)]
    fn cache_key_verification_timestamp(id: i32) -> String {
        format!("user:{}:verification", id)
    }
}

impl UserManager {
    pub async fn new(
        postgres: sqlx::Pool<sqlx::postgres::Postgres>,
        redis: redis::aio::MultiplexedConnection,
    ) -> error::ResultNew<Self> {
        let user_manager = Self { postgres, redis };

        // Initialize tables
        {
            user_manager
                .postgres
                .execute(sqlx::query(
                    "CREATE TABLE IF NOT EXISTS verifications (id SERIAL PRIMARY KEY)",
                ))
                .await
                .map_err(|err| {
                    error!(
                        "failed to create verifications table, due to database error: {}",
                        err
                    );
                    error::ErrorNew::SqlxError { inner: err }
                })?;

            user_manager
                .postgres.execute(sqlx::query("CREATE TABLE IF NOT EXISTS users (id SERIAL PRIMARY KEY, name VARCHAR UNIQUE NOT NULL, nickname VARCHAR, email VARCHAR UNIQUE, hash VARCHAR NOT NULL, rights VARCHAR NOT NULL, profile_picture VARCHAR)"))
                .await
                .map_err(|err| {
                    error!("failed to create users table, due to database error: {}", err);
                    error::ErrorNew::SqlxError { inner: err }
                })?;
        }

        if let Ok(0) = user_manager.user_count().await {
            let hash = helper::generate_hash("admin");

            user_manager
                .add_user(
                    "admin",
                    Some("Admin"),
                    None,
                    &hash,
                    model::UserRights::Admin,
                )
                .await
                .map_err(|err| error::ErrorNew::Error { inner: err.into() })?;
        }

        Ok(user_manager)
    }

    pub async fn user_count(&self) -> error::Result<usize> {
        let row = sqlx::query("SELECT count(*) as count FROM users")
            .fetch_one(&self.postgres)
            .await
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let count: i64 = row
            .try_get("count")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        Ok(count as usize)
    }

    pub async fn users(
        &self,
        include_members: bool,
        include_normal: bool,
        include_unauthenticated: bool,
    ) -> error::Result<Vec<model::User>> {
        let mut users = Vec::new();

        let mut query_builder = sqlx::QueryBuilder::new(
            "SELECT id, name, nickname, email, rights, profile_picture is not null as has_profile_picture FROM users WHERE id > 1 AND ",
        );

        query_builder.push("rights in (");
        {
            let mut separated = query_builder.separated(", ");

            if include_members {
                separated.push_bind(model::UserRights::Admin);
                separated.push_bind(model::UserRights::Maintainer);
                separated.push_bind(model::UserRights::Member);
            }

            if include_normal {
                separated.push_bind(model::UserRights::Normal);
            }

            if include_unauthenticated {
                separated.push_bind(model::UserRights::Unauthenticated);
            }
        }
        query_builder.push(")");
        query_builder.push(" ORDER BY name");

        let query = query_builder.build();

        let mut rows = query.fetch(&self.postgres);
        while let Some(row) = rows.try_next().await.map_err(|err| {
            error!("failed to get users, due to database error: {}", err);
            error::Error::SqlxError { inner: err }
        })? {
            let id: i32 = row
                .try_get("id")
                .map_err(|err| error::Error::SqlxError { inner: err })?;

            let name: &str = row
                .try_get("name")
                .map_err(|err| error::Error::SqlxError { inner: err })?;

            let nickname: Option<String> = row
                .try_get("nickname")
                .map_err(|err| error::Error::SqlxError { inner: err })?;

            let email: Option<String> = row
                .try_get("email")
                .map_err(|err| error::Error::SqlxError { inner: err })?;

            let rights: model::UserRights = row
                .try_get("rights")
                .map_err(|err| error::Error::SqlxError { inner: err })?;

            let has_profile_picture: bool = row
                .try_get("has_profile_picture")
                .map_err(|err| error::Error::SqlxError { inner: err })?;

            users.push(model::User {
                id,
                name: name.to_string(),
                nickname,
                email,
                rights,
                has_profile_picture,
            });
        }

        Ok(users)
    }

    pub async fn user(&self, id: i32) -> error::Result<model::User> {
        let row = sqlx::query("SELECT id, name, nickname, email, rights, profile_picture is not null as has_profile_picture FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&self.postgres)
            .await
            .map_err(|err| {
                error!(id = id, "failed to get user, due to database error: {}", err);
                error::Error::SqlxError { inner: err }
            })?;

        let id: i32 = row
            .try_get("id")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let name: String = row
            .try_get("name")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let nickname: Option<String> = row
            .try_get("nickname")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let email: Option<String> = row
            .try_get("email")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let rights: model::UserRights = row
            .try_get("rights")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let has_profile_picture: bool = row
            .try_get("has_profile_picture")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        Ok(model::User {
            id,
            name: name,
            nickname,
            email,
            rights: rights,
            has_profile_picture,
        })
    }

    pub async fn user_by_name(&self, name: &str) -> error::Result<model::User> {
        let row = sqlx::query("SELECT id, name, nickname, email, rights, profile_picture is not null as has_profile_picture FROM users WHERE name = $1")
            .bind(name)
            .fetch_one(&self.postgres)
            .await
            .map_err(|err| {
                error!(name = name, "failed to get user by name, due to database error: {}", err);
                error::Error::SqlxError { inner: err }
            })?;

        let id: i32 = row
            .try_get("id")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let name: String = row
            .try_get("name")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let nickname: Option<String> = row
            .try_get("nickname")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let email: Option<String> = row
            .try_get("email")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let rights: model::UserRights = row
            .try_get("rights")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let has_profile_picture: bool = row
            .try_get("has_profile_picture")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        Ok(model::User {
            id,
            name: name,
            nickname,
            email,
            rights: rights,
            has_profile_picture,
        })
    }

    pub async fn user_and_hash_by_name(&self, name: &str) -> error::Result<(model::User, String)> {
        let row = sqlx::query("SELECT id, name, nickname, email, rights, profile_picture is not null as has_profile_picture, hash FROM users WHERE name = $1")
            .bind(name)
            .fetch_one(&self.postgres)
            .await
            .map_err(|err| {
                error!(name = name, "failed to get user by name, due to database error: {}", err);
                error::Error::SqlxError { inner: err }
            })?;

        let id: i32 = row
            .try_get("id")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let name: String = row
            .try_get("name")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let nickname: Option<String> = row
            .try_get("nickname")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let email: Option<String> = row
            .try_get("email")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let rights: model::UserRights = row
            .try_get("rights")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let has_profile_picture: bool = row
            .try_get("has_profile_picture")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let hash: String = row
            .try_get("hash")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        Ok((
            model::User {
                id,
                name: name,
                nickname,
                email,
                rights: rights,
                has_profile_picture,
            },
            hash,
        ))
    }

    pub async fn user_profile_picture(&self, id: i32) -> error::Result<Option<Vec<u8>>> {
        let mut cache = self.redis.clone();

        // Read from cache
        let mut profile_picture: Option<Option<String>> = {
            redis::cmd("GET")
                .arg(Self::cache_key_profile_picture(id))
                .query_async(&mut cache)
                .await
                .unwrap_or_else(|err| {
                    error!(
                        id = id,
                        "failed to get user profile picture from cache, due to cache error: {}",
                        err
                    );
                    None
                })
        };

        // Read from database
        if profile_picture.is_none() {
            let pp = {
                let row = sqlx::query("SELECT profile_picture FROM users WHERE id = $1")
                    .bind(id)
                    .fetch_one(&self.postgres)
                    .await
                    .map_err(|err| {
                        error!(
                            id = id,
                            "failed to get user profile picture, due to database error: {}", err
                        );
                        error::Error::SqlxError { inner: err }
                    })?;

                row.try_get("profile_picture")
                    .map_err(|err| error::Error::SqlxError { inner: err })?
            };

            // Write to cache
            redis::cmd("SET")
                .arg(Self::cache_key_profile_picture(id))
                .arg(&pp)
                .exec_async(&mut cache)
                .await
                .unwrap_or_else(|err| {
                    error!(
                        id = id,
                        "failed to set user profile picture to cache, due to cache error: {}", err
                    );
                });

            profile_picture = Some(pp);
        }

        if let Some(Some(profile_picture)) = profile_picture {
            Ok(hex::decode(profile_picture).ok())
        } else {
            Ok(None)
        }
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

        let result =  sqlx::query(
                "INSERT INTO users (name, nickname, email, hash, rights) VALUES ($1, $2, $3, $4, $5) RETURNING id",
            )
            .bind(name)
            .bind(nickname)
            .bind(email)
            .bind(hash)
            .bind(&rights)
            .fetch_one(&self.postgres)
            .await;

        match result {
            Ok(row) => {
                let id: i32 = row
                    .try_get("id")
                    .map_err(|err| error::ErrorAddUser::SqlxError { inner: err })?;

                Ok(model::User {
                    id,
                    name: name.to_string(),
                    nickname: nickname.map(|x| x.to_string()),
                    email: email.map(|x| x.to_string()),
                    rights: rights,
                    has_profile_picture: false,
                })
            }
            Err(err) => {
                if let sqlx::Error::Database(err) = &err {
                    if let Some(constraint) = err.constraint() {
                        match constraint {
                            "users_name_key" => {
                                return Err(error::ErrorAddUser::NameIsTaken {
                                    name: name.to_string(),
                                });
                            }
                            "users_email_key" => {
                                return Err(error::ErrorAddUser::EMailIsTaken {
                                    email: email.unwrap_or_default().to_string(),
                                });
                            }
                            _ => {}
                        }
                    }
                }

                Err(error::ErrorAddUser::SqlxError { inner: err })
            }
        }
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

        let mut query_builder = sqlx::QueryBuilder::new("UPDATE users SET ");

        let mut separated = query_builder.separated(", ");

        let mut no_set = true;
        if let Some(name) = name {
            separated.push("name = ");
            separated.push_bind_unseparated(name);
            no_set = false;
        }
        if let Some(nickname) = nickname {
            separated.push("nickname = ");
            separated.push_bind_unseparated(nickname);
            no_set = false;
        }
        if let Some(email) = email {
            separated.push("email = ");
            separated.push_bind_unseparated(email);
            no_set = false;
        }
        if let Some(hash) = hash {
            separated.push("hash = ");
            separated.push_bind_unseparated(hash);
            no_set = false;
        }
        if let Some(rights) = rights {
            separated.push("rights = ");
            separated.push_bind_unseparated(rights);
            no_set = false;
        }
        if let Some(profile_picture) = profile_picture {
            separated.push("profile_picture = ");
            separated.push_bind_unseparated(hex::encode(profile_picture));
            no_set = false;
        }

        if no_set {
            return Ok(());
        }

        query_builder.push(" WHERE id = ");
        query_builder.push_bind(id);

        let query = query_builder.build();

        let result = self
            .postgres
            .execute(query)
            .await
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        if result.rows_affected() == 0 {
            error!(id = id, "failed to update user.");
            return Err(error::Error::OperationFailed {
                msg: "failed to update user.",
            });
        }

        // Invalidate cache
        {
            let mut conn = self.redis.clone();
            redis::cmd("DEL")
                .arg(Self::cache_key_profile_picture(id))
                .exec_async(&mut conn)
                .await
                .map_err(|err| error::Error::RedisError { inner: err })?;
        }

        Ok(())
    }

    pub async fn remove_user(&self, id: i32) -> error::Result<()> {
        if id == 1 {
            return Err(error::Error::OperationFailed {
                msg: "cannot remove admin user.",
            });
        }

        let result = self
            .postgres
            .execute(sqlx::query("DELETE FROM users WHERE id = $1").bind(id))
            .await
            .map_err(|err| {
                error!(
                    id = id,
                    "failed to remove user, due to database error: {}", err
                );
                error::Error::SqlxError { inner: err }
            })?;

        if result.rows_affected() == 0 {
            error!(id = id, "failed to remove user.");
            return Err(error::Error::OperationFailed {
                msg: "failed to remove user.",
            });
        }

        Ok(())
    }

    pub async fn has_verification(&self, id: i32) -> error::Result<bool> {
        let row = sqlx::query("SELECT count(*) as count FROM verifications WHERE id = $1")
            .bind(id)
            .fetch_one(&self.postgres)
            .await
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let count: i64 = row
            .try_get("count")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        Ok(count > 0)
    }

    pub async fn verification_timestamp(&self, id: i32) -> error::Result<Option<DateTime<Utc>>> {
        let mut conn = self.redis.clone();
        let timestamp: Option<String> = redis::cmd("GET")
            .arg(Self::cache_key_verification_timestamp(id))
            .query_async(&mut conn)
            .await
            .map_err(|err| error::Error::RedisError { inner: err })?;

        if let Some(timestamp) = timestamp {
            Ok(Some(timestamp.parse().map_err(|_| {
                error::Error::OperationFailed {
                    msg: "parse date time",
                }
            })?))
        } else {
            Ok(None)
        }
    }

    pub async fn set_verification_timestamp(
        &self,
        id: i32,
        datetime: DateTime<Utc>,
    ) -> error::Result<()> {
        let mut conn = self.redis.clone();
        redis::cmd("SET")
            .arg(Self::cache_key_verification_timestamp(id))
            .arg(datetime.to_rfc3339())
            .exec_async(&mut conn)
            .await
            .map_err(|err| error::Error::RedisError { inner: err })?;
        Ok(())
    }

    pub async fn add_verification(&self, id: i32) -> error::Result<()> {
        sqlx::query("INSERT INTO verifications (id) VALUES ($1)")
            .bind(id)
            .execute(&self.postgres)
            .await
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        Ok(())
    }

    pub async fn remove_verification(&self, id: i32) -> error::Result<()> {
        let result = self
            .postgres
            .execute(sqlx::query("DELETE FROM verifications WHERE id = $1").bind(id))
            .await
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        if result.rows_affected() == 0 {
            return Err(error::Error::OperationFailed {
                msg: "failed to remove verification.",
            });
        }

        Ok(())
    }
}
