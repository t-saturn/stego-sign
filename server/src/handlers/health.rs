use axum::{Json, extract::State, response::IntoResponse};
use serde::Serialize;
use tracing::info;

use crate::{AppState, config::db};

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
    version: &'static str,
    env: EnvInfo,
    db: DbHealth,
    storage: StorageHealth,
}

#[derive(Serialize)]
pub struct EnvInfo {
    server_host: String,
    server_port: u16,
    minio_endpoint: String,
}

#[derive(Serialize)]
pub struct DbHealth {
    status: bool,
    ping_ms: Option<f64>,
    host: String,
    port: u16,
    database: String,
    error: Option<String>,
}

#[derive(Serialize)]
pub struct StorageHealth {
    status: bool,
    endpoint: String,
    buckets: Vec<String>,
    error: Option<String>,
}

// -- GET /health
pub async fn health_handler(State(state): State<AppState>) -> impl IntoResponse {
    info!("health check requested");

    // -- db ping
    let db_health = match db::ping(&state.db).await {
        Ok(ms) => DbHealth {
            status: true,
            ping_ms: Some((ms * 100.0).round() / 100.0),
            host: state.env.postgres_host.clone(),
            port: state.env.postgres_port,
            database: state.env.postgres_db.clone(),
            error: None,
        },
        Err(e) => DbHealth {
            status: false,
            ping_ms: None,
            host: state.env.postgres_host.clone(),
            port: state.env.postgres_port,
            database: state.env.postgres_db.clone(),
            error: Some(e.to_string()),
        },
    };

    // -- minio ping: list buckets
    let storage_health = match state.storage.list_buckets().send().await {
        Ok(resp) => {
            let buckets = resp
                .buckets()
                .iter()
                .filter_map(|b| b.name().map(String::from))
                .collect::<Vec<_>>();
            StorageHealth {
                status: true,
                endpoint: state.env.minio_endpoint.clone(),
                buckets,
                error: None,
            }
        }
        Err(e) => StorageHealth {
            status: false,
            endpoint: state.env.minio_endpoint.clone(),
            buckets: vec![],
            error: Some(e.to_string()),
        },
    };

    Json(HealthResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
        env: EnvInfo {
            server_host: state.env.server_host.clone(),
            server_port: state.env.server_port,
            minio_endpoint: state.env.minio_endpoint.clone(),
        },
        db: db_health,
        storage: storage_health,
    })
}
