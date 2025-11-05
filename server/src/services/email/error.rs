use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorNew {
    #[error("lettre error: {inner}")]
    LettreError {
        inner: lettre::transport::smtp::Error,
    },
}
pub type ResultNew<T> = std::result::Result<T, ErrorNew>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("lettre error: {inner}")]
    LettreError { inner: lettre::transport::smtp::Error },
}
pub type Result<T> = std::result::Result<T, Error>;
