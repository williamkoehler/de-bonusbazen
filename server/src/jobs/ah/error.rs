use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error: {inner}")]
    DatabaseError {
        inner: crate::databases::postgres::error::Error,
    },
}
pub type Result<T> = std::result::Result<T, Error>;
