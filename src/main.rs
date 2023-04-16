use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, Request, StatusCode},
    routing::get,
    Router,
};

use tower::ServiceExt;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

use sqlx::postgres::{PgPool, PgPoolOptions};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::{net::SocketAddr, time::Duration};


#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "{{db_url}}".to_string());

    // setup connection pool
    let pool = PgPoolOptions::new()
        .max_connections({{db_max_connections}})
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    // build our application with some routes
    let app = Router::new()
        .route(
            "/",
            get(using_connection_pool_extractor).post(using_connection_extractor),
        )
        .with_state(pool);



    tokio::join!(
        serve(using_serve_dir(), 3001),
        serve(app.into_make_service(), 8080)
    );
}


fn using_serve_dir() -> Router {
    // serve the file in the "assets" directory under `/assets`
    Router::new().nest_service("/assets", ServeDir::new("assets"))
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([0, 0, 0, 0], {{port}}));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
      .serve(app.layer(TraceLayer::new_for_http()).into_make_service())
      .await
      .unwrap();
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

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
