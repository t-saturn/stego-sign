mod config;
mod routes;

use axum::Router;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::info;

use config::{Env, db, logger};

// -- shared application state injected into every handler
#[derive(Clone)]
pub struct AppState {
    pub env: Arc<Env>,
    pub db: Arc<sea_orm::DatabaseConnection>,
}

#[tokio::main]
async fn main() {
    // -- init logger first so every subsequent log is captured
    logger::init();

    // -- load env vars
    let env = Env::load();
    info!(host = %env.server_host, port = %env.server_port, "starting stego-server");

    // -- connect to database
    let db_conn = db::connect(&env)
        .await
        .expect("failed to establish database connection");

    let state = AppState {
        env: Arc::new(env.clone()),
        db: Arc::new(db_conn),
    };

    // -- build router with trace middleware (logs every request/response)
    let app = Router::new()
        .merge(routes::health::router())
        .with_state(state)
        .layer(
            TraceLayer::new_for_http().make_span_with(|req: &axum::http::Request<_>| {
                tracing::info_span!(
                    "http_request",
                    method = %req.method(),
                    uri    = %req.uri(),
                )
            }),
        );

    let addr = format!("{}:{}", env.server_host, env.server_port);
    info!(address = %addr, "server listening");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind address");

    axum::serve(listener, app).await.expect("server error");
}
