use std::sync::Arc;

use crate::{database::Database, posts::PostManager, users::UserManager};

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub user_manager: UserManager,
    pub post_manager: PostManager,
    pub config: Arc<Config>,
}

pub struct Config {
    pub jwt_expiry_time: u64,
    pub jwt_authentication_secret: String,
    pub jwt_verification_secret: String,
}
