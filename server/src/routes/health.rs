use axum::{Json, Router, extract::State, response::IntoResponse, routing::get};
use serde::Serialize;
use tracing::info;

use crate::AppState;
use crate::config::db;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
    db: DbHealth,
}

#[derive(Serialize)]
struct DbHealth {
    status: bool,
    ping_ms: Option<f64>,
    error: Option<String>,
}

// -- GET /health
async fn health_handler(State(state): State<AppState>) -> impl IntoResponse {
    info!("health check requested");

    let db_health = match db::ping(&state.db).await {
        Ok(ping_ms) => {
            info!(ping_ms = %ping_ms, "database ping successful");
            DbHealth {
                status: true,
                ping_ms: Some((ping_ms * 100.0).round() / 100.0),
                error: None,
            }
        }
        Err(e) => {
            tracing::warn!(error = %e, "database ping failed");
            DbHealth {
                status: false,
                ping_ms: None,
                error: Some(e.to_string()),
            }
        }
    };

    Json(HealthResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
        db: db_health,
    })
}

// -- returns the router for health endpoints
pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health_handler))
}
