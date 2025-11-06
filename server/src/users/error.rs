use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorNew {
    #[error("sqlx error: {inner}")]
    SqlxError { inner: sqlx::Error },

    #[error("error: {inner}")]
    Error {
        inner: Box<dyn std::error::Error + Send + Sync>,
    },
}
pub type ResultNew<T> = std::result::Result<T, ErrorNew>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("sqlx error: {inner}")]
    SqlxError { inner: sqlx::Error },

    #[error("redis error: {inner}")]
    RedisError { inner: redis::RedisError },

    #[error("invalid data: {msg}")]
    InvalidData { msg: &'static str },

    #[error("operation failed: {msg}")]
    OperationFailed { msg: &'static str },
}
pub type Result<T> = std::result::Result<T, Error>;


#[derive(Error, Debug)]
pub enum ErrorAddUser {
    #[error("internal error: {msg}")]
    InternalError { msg: String },
    
    #[error("sqlx error: {inner}")]
    SqlxError { inner: sqlx::Error },

    #[error("invalid name: {name}")]
    InvalidName { name: String },

    #[error("name is taken: {name}")]
    NameIsTaken { name: String },

    #[error("invalid nickname: {nickname}")]
    InvalidNickname { nickname: String },

    #[error("invalid email: {email}")]
    InvalidEMail { email: String },

    #[error("email is taken: {email}")]
    EMailIsTaken { email: String },

    #[error("invalid password")]
    InvalidPassword,
}
pub type ResultAddUser<T> = std::result::Result<T, ErrorAddUser>;
