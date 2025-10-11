use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorNew {
    #[error("redis error: {inner}")]
    RedisError { inner: redis::RedisError },
}
pub type ResultNew<T> = std::result::Result<T, ErrorNew>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("redis error: {inner}")]
    RedisError { inner: redis::RedisError },

    #[error("operation failed: {msg}")]
    OperationFailed { msg: &'static str },
}
pub type Result<T> = std::result::Result<T, Error>;