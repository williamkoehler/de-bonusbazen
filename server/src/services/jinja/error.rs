use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("template not found: {name}")]
    TemplateNotFound { name: String },
    
    #[error("jinja error: {inner}")]
    JinjaError { inner: minijinja::Error },
}
pub type Result<T> = std::result::Result<T, Error>;
