use tower_http::services::ServeDir;
use axum::Router;

pub fn using_serve_dir() -> Router {
  // serve the file in the "assets" directory under `/assets`
  Router::new().nest_service("/assets", ServeDir::new("assets"))
}