use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorNew {
    #[error("database error: {inner}")]
    CronSchedulerError {
        inner: tokio_cron_scheduler::JobSchedulerError,
    },
}
pub type ResultNew<T> = std::result::Result<T, ErrorNew>;
