use std::sync::Arc;

use tracing::*;

use crate::{
    databases::{postgres::PostgresDb, redis::RedisDb},
    services::{ah::AhService, email::EMailService},
    state::Config,
};

pub mod ah;
pub mod error;
pub mod user;

pub struct Jobs {
    job_scheduler: tokio_cron_scheduler::JobScheduler,

    pub ah_jobs: ah::AhJobs,
    pub user_jobs: user::UserJobs,
}

impl Jobs {
    pub async fn new(
        jobs_config: &crate::config::JobsConfig,
        config: Arc<Config>,
        postgres: PostgresDb,
        redis: RedisDb,
        ah_service: AhService,
        email_service: EMailService,
    ) -> error::ResultNew<Self> {
        let job_scheduler = tokio_cron_scheduler::JobScheduler::new()
            .await
            .map_err(|err| error::ErrorNew::CronSchedulerError { inner: err })?;

        let ah_jobs = ah::AhJobs::new(postgres.clone(), redis.clone(), ah_service.clone());
        let user_jobs = user::UserJobs::new(
            config.clone(),
            postgres.clone(),
            redis.clone(),
            email_service.clone(),
        );

        // Initialize default jobs
        {
            {
                let redis = redis.clone();
                let ah = ah_jobs.clone();

                tokio::spawn(async move {
                    let last_ah_refresh = redis.last_ah_refresh().await.unwrap_or_default();
                    let needs_ah_refresh = match last_ah_refresh {
                        Some(time) => {
                            chrono::Utc::now().signed_duration_since(time).num_hours() >= 24
                        }
                        None => true,
                    };
                    if needs_ah_refresh {
                        tokio::spawn(async move {
                            if let Err(err) = ah.update_ah_products_job().await {
                                error!("ah update products job failed: {}", err);
                            }
                        });
                    } else {
                        info!(
                            "AH products are up to date (last refresh: {})",
                            last_ah_refresh.unwrap()
                        );
                    }
                });
            }

            // Add AH refresh job
            {
                let ah_jobs = ah_jobs.clone();

                // Default: every 12 hours
                let schedule = jobs_config
                    .ah_refresh_cron
                    .clone()
                    .unwrap_or_else(|| "0 0 */12 * * *".to_string());

                job_scheduler
                    .add(
                        tokio_cron_scheduler::Job::new_async(schedule, move |_, _| {
                            let ah = ah_jobs.clone();
                            Box::pin(async move {
                                if let Err(err) = ah.update_ah_products_job().await {
                                    error!("ah update products job failed: {}", err);
                                }
                            })
                        })
                        .map_err(|err| error::ErrorNew::CronSchedulerError { inner: err })?,
                    )
                    .await
                    .map_err(|err| error::ErrorNew::CronSchedulerError { inner: err })?;
            }

            // Add failback user registration job
            {
                let user_jobs = user_jobs.clone();

                // Default: every hour
                let schedule = "0 0 * * * *".to_string();

                job_scheduler
                    .add(
                        tokio_cron_scheduler::Job::new_async(schedule, move |_, _| {
                            let user_jobs = user_jobs.clone();
                            Box::pin(async move {
                                if let Err(err) = user_jobs.handle_user_registrations_job().await {
                                    error!("user registration failback job failed: {}", err);
                                }
                            })
                        })
                        .map_err(|err| error::ErrorNew::CronSchedulerError { inner: err })?,
                    )
                    .await
                    .map_err(|err| error::ErrorNew::CronSchedulerError { inner: err })?;
            }
        }

        // Start job scheduler
        {
            let job_scheduler = job_scheduler.clone();
            tokio::spawn(async move {
                let _ = job_scheduler.start().await;
            });
        }

        Ok(Self {
            job_scheduler,
            ah_jobs,
            user_jobs,
        })
    }
}
