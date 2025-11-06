use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorNew {
    #[error("sqlx error: {inner}")]
    SqlxError { inner: sqlx::Error },
}
pub type ResultNew<T> = std::result::Result<T, ErrorNew>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("sqlx error: {inner}")]
    SqlxError { inner: sqlx::Error },

    #[error("invalid data: {msg}")]
    InvalidData { msg: &'static str },

    #[error("operation failed: {msg}")]
    OperationFailed { msg: &'static str },
}
pub type Result<T> = std::result::Result<T, Error>;
