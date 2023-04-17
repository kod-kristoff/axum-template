pub mod telemetry;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use tower_http::trace::TraceLayer;

pub fn app() -> Router {
    Router::new()
        .route("/healthz", get(health_check))
        .layer(TraceLayer::new_for_http())
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}
