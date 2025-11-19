use std::sync::Arc;

use axum::{Router, routing::*};
use rand::Rng;
use tracing::{level_filters::LevelFilter, *};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod config;
mod state;
pub use state::ArcState;

use crate::services::recaptcha;

mod jobs;
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

    // Global config
    let global_config =
        std::sync::Arc::new(state::Config {
            access_host: config.server.access_host.clone().unwrap_or_else(|| {
                format!("http://localhost:{}", config.server.port.unwrap_or(8080))
            }),
            jwt: state::JwtConfig {
                verification_secret: config
                    .server
                    .jwt
                    .verification_secret
                    .clone()
                    .unwrap_or_else(|| {
                        rand::rng()
                            .sample_iter(&rand::distr::Alphanumeric)
                            .take(20)
                            .map(char::from)
                            .collect()
                    }),
                authentication_secret: config
                    .server
                    .jwt
                    .authentication_secret
                    .clone()
                    .unwrap_or_else(|| {
                        rand::rng()
                            .sample_iter(&rand::distr::Alphanumeric)
                            .take(20)
                            .map(char::from)
                            .collect()
                    }),
                expiry_time: config.server.jwt.expire.unwrap_or(432000 /* 5 days */),
            },
            recaptcha: state::ReCaptchaConfig {
                site_key: config.server.recaptcha.site_key.clone(),
            },
        });

    // Initialize database connections
    let (postgres, redis) = {
        info!("connecting to postgres database...");

        let postgres = sqlx::postgres::PgPoolOptions::new()
            .max_connections(config.postgres.pool_max.unwrap_or(5))
            .connect(&config.postgres.url)
            .await?;

        info!("connecting to redis database...");

        let client = redis::Client::open(config.redis.url.as_str())?;
        let redis = client.get_multiplexed_tokio_connection().await?;

        (postgres, redis)
    };

    let ah_service = services::ah::AhService::new()?;
    let recaptcha_service = recaptcha::ReCaptchaService::new(&config.server.recaptcha)?;
    let email_service = services::email::EMailService::new(&config.server.email)?;
    let jinja_service = services::jinja::JinjaService::new(&config.server.jinja);

    let user_manager = users::UserManager::new(postgres.clone(), redis.clone()).await?;
    let post_manager = posts::PostManager::new(postgres.clone()).await?;
    let ah_manager = misc::ah::AhManager::new(postgres.clone(), redis.clone()).await?;

    let jobs = jobs::Jobs::new(
        &config.jobs.unwrap_or_default(),
        global_config.clone(),
        user_manager.clone(),
        ah_manager.clone(),
        ah_service.clone(),
        email_service.clone(),
        jinja_service.clone(),
    )
    .await?;

    let state = Arc::new(state::State {
        // Databases
        postgres,
        redis,

        // Services
        ah_service,
        recaptcha_service,
        email_service,
        jinja_service,

        // Jobs
        jobs,

        // Managers
        user_manager,
        post_manager,
        ah_manager,

        // Config
        config: global_config,
    });

    // Initialize http server
    let app = Router::new().nest(
        "/api",
        Router::new()
            .nest("/users", api::user::router())
            .nest("/posts", api::post::router())
            .nest("/ah", api::ah::router())
            .route("/check", get(api::auth::get_check))
            .layer(axum::middleware::from_fn_with_state(
                state.clone(),
                api::middleware::auth::auth_middleware,
            ))
            .route("/recaptcha", get(api::auth::get_recaptcha))
            .route("/login", post(api::auth::post_login))
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
