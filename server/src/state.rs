use std::sync::Arc;

use crate::*;

pub type ArcState = Arc<State>;

pub struct State {
    pub postgres: sqlx::Pool<sqlx::postgres::Postgres>,
    pub redis: redis::aio::MultiplexedConnection,

    pub ah_service: services::ah::AhService,
    pub recaptcha_service: services::recaptcha::ReCaptchaService,
    pub email_service: services::email::EMailService,
    pub jinja_service: services::jinja::JinjaService,

    pub jobs: jobs::Jobs,

    pub user_manager: users::UserManager,
    pub post_manager: posts::PostManager,
    pub ah_manager: misc::ah::AhManager,

    pub config: Arc<Config>,
}

pub struct Config {
    pub access_host: String,
    pub jwt: JwtConfig,
    pub recaptcha: ReCaptchaConfig,
}

pub struct JwtConfig {
    pub expiry_time: u64,
    pub authentication_secret: String,
    pub verification_secret: String,
}

pub struct ReCaptchaConfig {
    pub site_key: String,
}
