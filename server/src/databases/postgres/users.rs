use futures_util::TryStreamExt;
use sqlx::{Executor, Row};

use crate::databases::postgres::model::*;

use super::error;
use super::model;

impl super::PostgresDb {
    pub async fn user_count(&self) -> error::Result<usize> {
        let row = sqlx::query("SELECT count(*) as count FROM users")
            .fetch_one(&self.pool)
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
    ) -> error::Result<Vec<model::RawUser>> {
        let mut users = Vec::new();

        let mut query_builder = sqlx::QueryBuilder::new(
            "SELECT id, name, nickname, email, rights, profile_picture is not null as has_profile_picture FROM users WHERE id > 1 AND ",
        );

        query_builder.push("rights in (");
        {
            let mut separated = query_builder.separated(", ");

            if include_members {
                separated.push_bind(RawUserRights::Admin);
                separated.push_bind(RawUserRights::Maintainer);
                separated.push_bind(RawUserRights::Member);
            }
            if include_normal {
                separated.push_bind(RawUserRights::Member);
            }
        }
        query_builder.push(")");
        query_builder.push(" ORDER BY name");

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

            let name: &str = row
                .try_get("name")
                .map_err(|err| error::Error::SqlxError { inner: err })?;

            let nickname: Option<String> = row
                .try_get("nickname")
                .map_err(|err| error::Error::SqlxError { inner: err })?;

            let email: Option<String> = row
                .try_get("email")
                .map_err(|err| error::Error::SqlxError { inner: err })?;

            let rights: RawUserRights = row
                .try_get("rights")
                .map_err(|err| error::Error::SqlxError { inner: err })?;

            let has_profile_picture: bool = row
                .try_get("has_profile_picture")
                .map_err(|err| error::Error::SqlxError { inner: err })?;

            users.push(model::RawUser {
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

    pub async fn user(&self, id: i32) -> error::Result<model::RawUser> {
        let row = sqlx::query("SELECT id, name, nickname, email, rights, profile_picture is not null as has_profile_picture FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| error::Error::SqlxError { inner: err })?;

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

        let rights: RawUserRights = row
            .try_get("rights")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let has_profile_picture: bool = row
            .try_get("has_profile_picture")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        Ok(model::RawUser {
            id,
            name: name,
            nickname,
            email,
            rights: rights,
            has_profile_picture,
        })
    }

    pub async fn user_by_name(&self, name: &str) -> error::Result<model::RawUser> {
        let row = sqlx::query("SELECT id, name, nickname, email, rights, profile_picture is not null as has_profile_picture FROM users WHERE name = $1")
            .bind(name)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| error::Error::SqlxError { inner: err })?;

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

        let rights: RawUserRights = row
            .try_get("rights")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let has_profile_picture: bool = row
            .try_get("has_profile_picture")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        Ok(model::RawUser {
            id,
            name,
            nickname,
            email,
            rights: rights,
            has_profile_picture,
        })
    }

    pub async fn user_and_hash_by_name(
        &self,
        name: &str,
    ) -> error::Result<(model::RawUser, String)> {
        let row = sqlx::query("SELECT id, name, nickname, email, hash, rights, profile_picture is not null as has_profile_picture FROM users WHERE name = $1")
            .bind(name)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| error::Error::SqlxError { inner: err })?;

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

        let hash: String = row
            .try_get("hash")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let rights: RawUserRights = row
            .try_get("rights")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let has_profile_picture: bool = row
            .try_get("has_profile_picture")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        Ok((
            model::RawUser {
                id,
                name,
                nickname,
                email,
                rights: rights,
                has_profile_picture,
            },
            hash,
        ))
    }

    pub async fn user_profile_picture(&self, id: i32) -> error::Result<Option<Vec<u8>>> {
        let row = sqlx::query("SELECT profile_picture FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let profile_picture: Option<String> = row
            .try_get("profile_picture")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        if let Some(profile_picture) = profile_picture {
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
        hash: &str,
        rights: RawUserRights,
    ) -> error::ResultAddUser<model::RawUser> {
        let result =  sqlx::query(
            "INSERT INTO users (name, nickname, email, hash, rights) VALUES ($1, $2, $3, $4, $5) RETURNING id",
        )
        .bind(name)
        .bind(nickname)
        .bind(email)
        .bind(hash)
        .bind(&rights)
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(row) => {
                let id: i32 = row
                    .try_get("id")
                    .map_err(|err| error::ErrorAddUser::SqlxError { inner: err })?;

                Ok(model::RawUser {
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
                                return Err(error::ErrorAddUser::UniqueNameConstraintViolation);
                            }
                            "users_email_key" => {
                                return Err(error::ErrorAddUser::UniqueEMailConstraintViolation);
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
        mut rights: Option<RawUserRights>,
        profile_picture: Option<&[u8]>,
    ) -> error::Result<()> {
        if id == 1 {
            rights.take();
        }

        let mut query_builder = sqlx::QueryBuilder::new("UPDATE users SET ");

        let mut separated = query_builder.separated(", ");

        if let Some(name) = name {
            separated.push("name = ");
            separated.push_bind_unseparated(name);
        }
        if let Some(nickname) = nickname {
            separated.push("nickname = ");
            separated.push_bind_unseparated(nickname);
        }
        if let Some(email) = email {
            separated.push("email = ");
            separated.push_bind_unseparated(email);
        }
        if let Some(rights) = rights {
            separated.push("rights = ");
            separated.push_bind_unseparated(rights);
        }
        if let Some(profile_picture) = profile_picture {
            separated.push("profile_picture = ");
            separated.push_bind_unseparated(hex::encode(profile_picture));
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

    pub async fn remove_user(&self, id: i32) -> error::Result<()> {
        if id == 1 {
            return Err(error::Error::OperationFailed {
                msg: "cannot remove admin user.",
            });
        }

        let result = self
            .pool
            .execute(sqlx::query("DELETE FROM users WHERE id = $1").bind(id))
            .await
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        if result.rows_affected() == 0 {
            return Err(error::Error::OperationFailed {
                msg: "failed to remove user.",
            });
        }

        Ok(())
    }
}
