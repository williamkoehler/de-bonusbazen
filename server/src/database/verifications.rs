use sqlx::{Executor, Row};

use super::error;

impl super::Database {
    pub async fn has_verification(&self, id: i32) -> error::Result<bool> {
        let row = sqlx::query("SELECT count(*) as count FROM verifications WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        let count: i64 = row
            .try_get("count")
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        Ok(count > 0)
    }

    pub async fn add_verification(&self, id: i32) -> error::Result<()> {
        sqlx::query("INSERT INTO verifications (id) VALUES ($1)")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|err| error::Error::SqlxError { inner: err })?;

        Ok(())
    }

    pub async fn remove_verification(&self, id: i32) -> error::Result<()> {
        let result = self
            .pool
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
