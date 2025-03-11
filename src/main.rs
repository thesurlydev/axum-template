use axum::{
  routing::get,
  Router,
};

use tokio::net::TcpListener;

{% if static_support %}
mod static_support;
use static_support::using_serve_dir;
{% endif %}

use tower_http::trace::TraceLayer;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use tokio::signal;

{% if db_support %}
mod db;
use db::{postgres};
use sqlx::postgres::PgPool;
{% endif %}


#[tokio::main]
async fn main() {
  tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer())
    .init();

  // build our application with some routes
  {% if db_support %}
  let pool = db::postgres::get_db_pool();
  let app = Router::new()
    .route("/", get(db::postgres::using_connection_pool_extractor).post(using_connection_extractor))
    .with_state(pool);
  {% else %}
  let app = Router::new().route("/", get(handler));
  {% endif %}


  tokio::join!(
        {% if static_support %}
        serve(using_serve_dir(), {{static_assets_port}}),
        {% endif %}
        serve(app, {{port}})
    );
}

async fn handler() -> &'static str {
  "Hello, world!"
}

async fn serve(app: Router, port: u16) {
  let addr_str = format!("[::]:{}", port);
  tracing::info!("listening on {}", addr_str);
  let listener = TcpListener::bind(addr_str.as_str()).await.expect("failed to bind");
  axum::serve(listener, app.layer(TraceLayer::new_for_http()).into_make_service())
      .with_graceful_shutdown(shutdown_signal())
      .await
      .unwrap();
}

// graceful shutdown
async fn shutdown_signal() {
  let ctrl_c = async {
    signal::ctrl_c()
      .await
      .expect("failed to install Ctrl+C handler");
  };

  #[cfg(unix)]
    let terminate = async {
    signal::unix::signal(signal::unix::SignalKind::terminate())
      .expect("failed to install signal handler")
      .recv()
      .await;
  };

  #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

  tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

  println!("signal received, starting graceful shutdown");
}
