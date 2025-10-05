use axum::{Router, routing::*};
use rand::Rng;
use tracing::{level_filters::LevelFilter, *};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod config;
mod state;
pub use state::AppState;

mod database;
mod services;

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
    let database = database::Database::new(&config.database).await?;

    let user_manager = users::UserManager::new(database.clone()).await?;
    let post_manager = posts::PostManager::new(database.clone()).await?;

    let state = AppState {
        database,
        user_manager,
        post_manager,
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
    };

    // Initialize http server
    let app = Router::new().nest(
        "/api",
        Router::new()
            .nest("/users", api::user::router())
            .nest("/posts", api::post::router())
            .layer(axum::middleware::from_fn_with_state(
                state.clone(),
                api::middleware::auth::auth_middleware,
            ))
            .route("/login", post(api::auth::login))
            .route("/register", post(api::auth::post_register))
            .route("/register/{token}", get(api::auth::get_verify))
            .with_state(state),
    );

    let addr: std::net::SocketAddr =
        format!("{}:{}", config.server.host, config.server.port).parse()?;
    info!("listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
