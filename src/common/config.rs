use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use std::time::Duration;
use tokio::time::sleep;

/// Config is a struct that holds the configuration for the application.
#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub database_max_connections: u32,
    pub database_min_connections: u32,

    pub service_host: String,
    pub service_port: String,

    /// CORS allowed origins. Use "*" to allow any origin, or a comma-separated list of origins.
    pub cors_allowed_origins: String,

    /// Request timeout in seconds.
    pub request_timeout_secs: u64,
}

/// from_env reads the environment variables and returns a Config struct.
/// It uses the dotenv crate to load environment variables from a .env file if it exists.
/// It returns a Result with the Config struct or an error if any of the environment variables are missing.
impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenvy::dotenv().ok();

        Ok(Self {
            database_url: env::var("DATABASE_URL")?,

            database_max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .map(|s| s.parse::<u32>().unwrap_or(5))
                .unwrap_or(5),
            database_min_connections: env::var("DATABASE_MIN_CONNECTIONS")
                .map(|s| s.parse::<u32>().unwrap_or(1))
                .unwrap_or(1),

            service_host: env::var("SERVICE_HOST")?,
            service_port: env::var("SERVICE_PORT")?,

            cors_allowed_origins: env::var("CORS_ALLOWED_ORIGINS").unwrap_or_else(|_| "*".to_string()),

            request_timeout_secs: env::var("REQUEST_TIMEOUT_SECS")
                .map(|s| s.parse::<u64>().unwrap_or(5))
                .unwrap_or(5),
        })
    }
}

/// setup_database initializes the database connection pool.
pub async fn setup_database(config: &Config) -> Result<PgPool, sqlx::Error> {
    // Attempt to connect repeatedly, with a small delay, until success (or a max number of tries)
    let mut attempts = 0;
    let pool = loop {
        attempts += 1;
        match PgPoolOptions::new()
            .max_connections(config.database_max_connections)
            .min_connections(config.database_min_connections)
            .connect(&config.database_url)
            .await
        {
            Ok(pool) => break pool,
            Err(err) => {
                if attempts >= 3 {
                    return Err(err);
                }
                eprintln!(
                    "Postgres not ready yet ({:?}), retrying in 1s… (attempt {}/{})",
                    err, attempts, 3
                );
                sleep(Duration::from_secs(1)).await;
            }
        }
    };

    Ok(pool)
}
