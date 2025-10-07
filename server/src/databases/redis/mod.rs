pub mod error;

pub mod users;

#[derive(Clone)]
pub struct RedisDb {
    multiplexed_connection: redis::aio::MultiplexedConnection,
}

impl RedisDb {
    pub async fn new(config: &crate::config::RedisDbConfig) -> error::ResultNew<RedisDb> {
        let client = redis::Client::open(config.url.as_str())
            .map_err(|err| error::ErrorNew::RedisError { inner: err })?;
        let multiplexed_connection = client
            .get_multiplexed_tokio_connection()
            .await
            .map_err(|err| error::ErrorNew::RedisError { inner: err })?;

        Ok(Self { multiplexed_connection })
    }
}
