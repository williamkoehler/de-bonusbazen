use std::sync::Arc;

use axum::{Router, routing::*};
use rand::Rng;
use tracing::{level_filters::LevelFilter, *};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod config;
mod state;
pub use state::ArcState;

mod databases;
mod services;

mod misc;
mod posts;
mod users;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (console_layer, _guard) = {
        let (stdout_writer, guard) = tracing_appender::non_blocking(std::io::stdout());

        let layer = tracing_subscriber::fmt::layer()
            .with_writer(stdout_writer)
            .with_ansi(true) // colored output
            .with_target(true);

        (layer, guard)
    };

    // Initialize environment filter
    let env_filter = tracing_subscriber::EnvFilter::from_default_env()
        .add_directive("sqlx=warn".parse().unwrap())
        .add_directive(LevelFilter::TRACE.into());

    // Initialize logging
    tracing_subscriber::registry()
        .with(env_filter)
        .with(console_layer)
        .init();

    // Load config
    let config = crate::config::load_config()?;

    // Initialize server
    let postgres = databases::postgres::PostgresDb::new(&config.postgres).await?;
    let redis = databases::redis::RedisDb::new(&config.redis).await?;

    let ah_service = services::ah::AhService::new()?;
    let ah_manager = misc::ah::AhManager::new(ah_service.clone(), postgres.clone());

    // ah_manager.refresh_ah_products().await;

    let user_manager = users::UserManager::new(postgres.clone()).await?;
    let post_manager = posts::PostManager::new(postgres.clone()).await?;

    let job_scheduler = tokio_cron_scheduler::JobScheduler::new().await?;

    let state = Arc::new(state::State {
        // Databases
        postgres,
        redis,

        // Services
        ah_service,

        // Managers
        user_manager,
        post_manager,
        ah_manager,

        // Global config
        config: std::sync::Arc::new(state::Config {
            jwt_verification_secret: config.server.jwt.verification_secret.unwrap_or_else(|| {
                rand::rng()
                    .sample_iter(&rand::distr::Alphanumeric)
                    .take(20)
                    .map(char::from)
                    .collect()
            }),
            jwt_authentication_secret: config.server.jwt.authentication_secret.unwrap_or_else(
                || {
                    rand::rng()
                        .sample_iter(&rand::distr::Alphanumeric)
                        .take(20)
                        .map(char::from)
                        .collect()
                },
            ),
            jwt_expiry_time: config.server.jwt.expire.unwrap_or(432000 /* 5 days */),
        }),
    });

    // Prepare database
    {
        let last_ah_refresh = state.redis.last_ah_refresh().await?;
        let needs_ah_refresh = match last_ah_refresh {
            Some(time) => chrono::Utc::now().signed_duration_since(time).num_hours() >= 2,
            None => true,
        };
        if needs_ah_refresh {
            info!("refreshing AH products...");
            state.ah_manager.refresh_ah_products().await;
            if let Err(err) = state.redis.set_last_ah_refresh(chrono::Utc::now()).await {
                error!("failed to update AH refresh time: {}", err);
            }
        } else {
            info!(
                "AH products are up to date (last refresh: {})",
                last_ah_refresh.unwrap()
            );
        }
    }

    // Schedule jobs
    {
        let jobs_config = config.jobs.unwrap_or_default();

        {
            let state = state.clone();

            let ah_refresh_cron = jobs_config
                .ah_refresh_cron
                .unwrap_or_else(|| "0 0 */6 * * *".to_string()); // Default: every six hours
            job_scheduler
                .add(tokio_cron_scheduler::Job::new_async(
                    ah_refresh_cron,
                    move |_, _| {
                        let state = state.clone();
                        Box::pin(async move {
                            state.ah_manager.refresh_ah_products().await;
                            if let Err(err) =
                                state.redis.set_last_ah_refresh(chrono::Utc::now()).await
                            {
                                error!("failed to update AH refresh time: {}", err);
                            }
                        })
                    },
                )?)
                .await?;
        }

        tokio::spawn(async move {
            let _ = job_scheduler.start().await;
        });
    }

    // Initialize http server
    let app = Router::new().nest(
        "/api",
        Router::new()
            .nest("/users", api::user::router())
            .nest("/posts", api::post::router())
            .nest("/ah", api::ah::router())
            .layer(axum::middleware::from_fn_with_state(
                state.clone(),
                api::middleware::auth::auth_middleware,
            ))
            .route("/login", post(api::auth::login))
            .route("/register", post(api::auth::post_register))
            .route("/register/{token}", get(api::auth::get_verify))
            .with_state(state),
    );

    let addr: std::net::SocketAddr = format!(
        "{}:{}",
        config.server.host.unwrap_or_else(|| "0.0.0.0".to_string()),
        config.server.port.unwrap_or(8080)
    )
    .parse()?;
    info!("listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
