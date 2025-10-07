use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub postgres: PostgresDbConfig,
    pub redis: RedisDbConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,

    pub jwt: ServerJwtConfig,
    // pub email: ServerEMailConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerJwtConfig {
    pub verification_secret: Option<String>,
    pub authentication_secret: Option<String>,
    pub expire: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct ServerEMailConfig {
    pub smtp_server: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_address: String,
}

#[derive(Debug, Deserialize)]
pub struct PostgresDbConfig {
    pub url: String,
    pub pool_max: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct RedisDbConfig {
    pub url: String,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv(); // Ignore error if no .env present

    let mut config_builder = config::Config::builder()
        .add_source(config::File::new("config/default.yaml", config::FileFormat::Yaml).required(false))
        .add_source(config::File::new("config/default.toml", config::FileFormat::Toml).required(false));

    if let Ok(env_name) = std::env::var("APP_ENV") {
        let path = format!("config/{}", env_name);
        config_builder = config_builder.add_source(config::File::with_name(&path).required(false));
    }

    config_builder =
        config_builder.add_source(config::Environment::with_prefix("APP").separator("__"));

    let config = config_builder.build()?;
    Ok(config.try_deserialize()?)
}
