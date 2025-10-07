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

    #[error("operation failed: {msg}")]
    OperationFailed { msg: &'static str },

    #[error("unique constraint violation: {key}")]
    UniqueConstraintViolation { key: String },
}
pub type Result<T> = std::result::Result<T, Error>;



#[derive(Error, Debug)]
pub enum ErrorAddUser {
    #[error("sqlx error: {inner}")]
    SqlxError { inner: sqlx::Error },

    #[error("unique name constraint violation")]
    UniqueNameConstraintViolation,

    #[error("unique email constraint violation")]
    UniqueEMailConstraintViolation,
}
pub type ResultAddUser<T> = std::result::Result<T, ErrorAddUser>;
