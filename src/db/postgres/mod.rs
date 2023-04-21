
use axum::{
  async_trait,
  extract::{FromRef, FromRequestParts, State},
  http::{request::Parts, StatusCode},
  routing::get,
  Router,
};

use sqlx::postgres::{PgPool, PgPoolOptions};

async fn get_db_pool() -> PgPool {
  let db_connection_str = std::env::var("DATABASE_URL")
    .unwrap_or_else(|_| "{{db_url}}".to_string());

  // setup connection pool
  return PgPoolOptions::new()
    .max_connections({{db_max_connections}})
    .acquire_timeout(Duration::from_secs(3))
    .connect(&db_connection_str)
    .await
    .expect("can't connect to database");
}

// we can extract the connection pool with `State`
async fn using_connection_pool_extractor(
  State(pool): State<PgPool>,
) -> Result<String, (StatusCode, String)> {
  sqlx::query_scalar("select 'hello world from pg'")
    .fetch_one(&pool)
    .await
    .map_err(internal_error)
}

// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
struct DatabaseConnection(sqlx::pool::PoolConnection<sqlx::Postgres>);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
  where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
  type Rejection = (StatusCode, String);

  async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
    let pool = PgPool::from_ref(state);
    let conn = pool.acquire().await.map_err(internal_error)?;
    Ok(Self(conn))
  }
}

async fn using_connection_extractor(
  DatabaseConnection(conn): DatabaseConnection,
) -> Result<String, (StatusCode, String)> {
  let mut conn = conn;
  sqlx::query_scalar("select 'hello world from pg'")
    .fetch_one(&mut conn)
    .await
    .map_err(internal_error)
}
