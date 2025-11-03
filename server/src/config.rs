use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub jobs: Option<JobsConfig>,
    pub postgres: PostgresDbConfig,
    pub redis: RedisDbConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: Option<String>,
    pub port: Option<u16>,

    pub jwt: ServerJwtConfig,
    pub recaptcha: ServerReCaptchaConfig,
    // pub email: ServerEMailConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerJwtConfig {
    pub verification_secret: Option<String>,
    pub authentication_secret: Option<String>,
    pub expire: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerReCaptchaConfig {
    pub secret_key: String,
    pub site_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerEMailConfig {
    pub smtp_server: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_address: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct JobsConfig {
    pub ah_refresh_cron: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostgresDbConfig {
    pub url: String,
    pub pool_max: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedisDbConfig {
    pub url: String,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv(); // Ignore error if no .env present

    let mut config_builder = config::Config::builder()
        .add_source(
            config::File::new("config/default.yaml", config::FileFormat::Yaml).required(false),
        )
        .add_source(
            config::File::new("config/default.toml", config::FileFormat::Toml).required(false),
        );

    if let Ok(env_name) = std::env::var("APP_ENV") {
        let path = format!("config/{}", env_name);
        config_builder = config_builder.add_source(config::File::with_name(&path).required(false));
    }

    config_builder =
        config_builder.add_source(config::Environment::with_prefix("APP").separator("__"));

    let config = config_builder.build()?;
    let config = config.try_deserialize()?;

    println!("Loaded config: {}", serde_json::to_string(&config).unwrap());
    Ok(config)
}
