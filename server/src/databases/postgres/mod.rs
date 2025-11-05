use sqlx::Executor;
use tracing::*;

pub mod error;
pub mod model;

pub mod posts;
pub mod users;
pub mod verifications;

pub mod ah;

#[derive(Clone)]
pub struct PostgresDb {
    pool: sqlx::Pool<sqlx::postgres::Postgres>,
}

impl PostgresDb {
    pub async fn new(config: &crate::config::PostgresDbConfig) -> error::ResultNew<PostgresDb> {
        info!("connecting to postgres database...");
        
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(config.pool_max.unwrap_or(5))
            .connect(&config.url)
            .await
            .map_err(|err| error::ErrorNew::SqlxError { inner: err })?;

        pool.execute(sqlx::query("CREATE TABLE IF NOT EXISTS verifications (id SERIAL PRIMARY KEY)"))
            .await
            .map_err(|err| {
                error!("failed to create verifications table: {}", err);
                error::ErrorNew::SqlxError { inner: err }
            })?;

        pool.execute(sqlx::query("CREATE TABLE IF NOT EXISTS users (id SERIAL PRIMARY KEY, name VARCHAR UNIQUE NOT NULL, nickname VARCHAR, email VARCHAR UNIQUE, hash VARCHAR NOT NULL, rights VARCHAR NOT NULL, profile_picture VARCHAR)"))
            .await
            .map_err(|err| {
                error!("failed to create users table: {}", err);
                error::ErrorNew::SqlxError { inner: err }
            })?;

        pool.execute(sqlx::query("CREATE TABLE IF NOT EXISTS posts (id SERIAL PRIMARY KEY, visibility VARCHAR NOT NULL, created_at TIMESTAMPTZ NOT NULL, updated_at TIMESTAMPTZ NOT NULL, author INTEGER, title VARCHAR NOT NULL, metadata JSONB, body TEXT)"))
            .await
            .map_err(|err|{
                error!("failed to create posts table: {}", err);
                error::ErrorNew::SqlxError { inner: err }
            })?;
        
        pool.execute(sqlx::query("CREATE TABLE IF NOT EXISTS ah_products (id BIGINT PRIMARY KEY, ranking BIGINT, data JSONB NOT NULL)"))
            .await
            .map_err(|err|{
                error!("failed to create ah_products table: {}", err);
                error::ErrorNew::SqlxError { inner: err }
            })?;
        
        pool.execute(sqlx::query("CREATE TABLE IF NOT EXISTS ah_comments (id SERIAL PRIMARY KEY, product_id BIGINT REFERENCES ah_products(id) ON DELETE CASCADE, user_id INTEGER REFERENCES users(id) ON DELETE CASCADE, comment VARCHAR NOT NULL)"))
            .await
            .map_err(|err|{
                error!("failed to create ah_products table: {}", err);
                error::ErrorNew::SqlxError { inner: err }
            })?;

        Ok(Self { pool })
    }
}
