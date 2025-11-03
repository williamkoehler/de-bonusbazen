use std::sync::Arc;

use crate::*;

pub type ArcState = Arc<State>;

pub struct State {
    pub postgres: databases::postgres::PostgresDb,
    pub redis: databases::redis::RedisDb,

    pub ah_service: services::ah::AhService,
    pub ah_manager: misc::ah::AhManager,
    pub recaptcha_service: services::recaptcha::ReCaptchaService,

    pub user_manager: users::UserManager,
    pub post_manager: posts::PostManager,

    pub config: Arc<Config>,
}

pub struct Config {
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