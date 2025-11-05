use chrono::Utc;

use crate::databases::redis::RedisDb;

use super::error;

impl RedisDb {
    pub async fn user_registration_timestamp(
        &self,
        user_id: i32,
    ) -> error::Result<Option<chrono::DateTime<Utc>>> {
        let mut conn = self.multiplexed_connection.clone();
        let timestamp: Option<String> = redis::cmd("GET")
            .arg(format!("user:{}:registration", user_id))
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

    pub async fn set_user_registration_timestamp(
        &self,
        user_id: i32,
        datetime: chrono::DateTime<Utc>,
    ) -> error::Result<()> {
        let mut conn = self.multiplexed_connection.clone();
        redis::cmd("SET")
            .arg(format!("user:{}:registration", user_id))
            .arg(datetime.to_rfc3339())
            .exec_async(&mut conn)
            .await
            .map_err(|err| error::Error::RedisError { inner: err })?;
        Ok(())
    }
}
