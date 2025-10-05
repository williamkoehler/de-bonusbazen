use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorNew {
    #[error("error: {inner}")]
    Error {
        inner: Box<dyn std::error::Error + Send + Sync>,
    },
}
pub type ResultNew<T> = std::result::Result<T, ErrorNew>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error: {inner}")]
    DatabaseError {
        inner: crate::database::error::Error,
    },

    #[error("invalid data: {msg}")]
    InvalidData { msg: &'static str },
}
pub type Result<T> = std::result::Result<T, Error>;


#[derive(Error, Debug)]
pub enum ErrorAddUser {
    #[error("database error: {inner}")]
    DatabaseError {
        inner: crate::database::error::ErrorAddUser,
    },

    #[error("invalid name: {name}")]
    InvalidName { name: String },

    #[error("name is taken: {name}")]
    NameIsTaken { name: String },

    #[error("invalid email: {email}")]
    InvalidEMail { email: String },

    #[error("email is taken: {email}")]
    EMailIsTaken { email: String },
}
pub type ResultAddUser<T> = std::result::Result<T, ErrorAddUser>;
