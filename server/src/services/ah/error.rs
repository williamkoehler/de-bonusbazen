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
    #[error("url error: {inner}")]
    UrlError { inner: url::ParseError },

    #[error("reqwest error: {inner}")]
    ReqwestError { inner: reqwest::Error },
}
pub type Result<T> = std::result::Result<T, Error>;
