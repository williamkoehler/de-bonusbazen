use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorNew {
    #[error("database error: {inner}")]
    DatabaseError {
        inner: crate::databases::postgres::error::Error,
    },
}
pub type ResultNew<T> = std::result::Result<T, ErrorNew>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error: {inner}")]
    DatabaseError {
        inner: crate::databases::postgres::error::Error,
    },

    #[error("invalid data: {msg}")]
    InvalidData { msg: &'static str },
}
pub type Result<T> = std::result::Result<T, Error>;
