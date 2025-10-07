use std::sync::Arc;

use crate::*;

pub type ArcState = Arc<State>;

pub struct State {
    pub postgres: databases::postgres::PostgresDb,
    pub redis: databases::redis::RedisDb,

    pub ah_service: services::ah::AhService,

    pub user_manager: users::UserManager,
    pub post_manager: posts::PostManager,

    pub config: Arc<Config>,
}

pub struct Config {
    pub jwt_expiry_time: u64,
    pub jwt_authentication_secret: String,
    pub jwt_verification_secret: String,
}
