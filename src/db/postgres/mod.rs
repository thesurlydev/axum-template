use axum::{
  extract::{State},
  http::{StatusCode},
};
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use sqlx::postgres::PgPool;


// we can extract the connection pool with `State`
pub(crate) async fn using_connection_pool_extractor(
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
  DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<String, (StatusCode, String)> {
  sqlx::query_scalar("select 'hello world from pg'")
      .fetch_one(&mut *conn)
      .await
      .map_err(internal_error)
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
  (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
