use axum::{Json, extract::State, response::IntoResponse};
use serde::Serialize;
use tracing::info;

use crate::{
    AppState,
    models::response::ApiResponse,
    repositories::{audit_log as audit_repo, document as doc_repo},
    services::storage,
};

#[derive(Debug, Serialize)]
pub struct StatsData {
    pub documents_signed: u64,
    pub verifications: u64,
    pub tampered: u64,
    pub storage_vaults: u64,
    pub objects: u64,
}

pub async fn stats_handler(State(state): State<AppState>) -> impl IntoResponse {
    let documents_signed = doc_repo::count_all(&state.db).await.unwrap_or(0);

    let verifications = audit_repo::count_by_action(&state.db, "VERIFY")
        .await
        .unwrap_or(0);

    let tampered = doc_repo::count_by_status(&state.db, "TAMPERED")
        .await
        .unwrap_or(0);

    let buckets = [
        storage::BUCKET_UPLOADS,
        storage::BUCKET_SIGNATURES,
        storage::BUCKET_CORRUPTED,
    ];

    let mut objects: u64 = 0;
    for bucket in &buckets {
        if let Ok(count) = storage::count_objects(&state.storage, bucket).await {
            objects += count;
        }
    }

    info!(
        documents_signed,
        verifications, tampered, objects, "stats requested"
    );

    Json(ApiResponse::ok(StatsData {
        documents_signed,
        verifications,
        tampered,
        storage_vaults: 3,
        objects,
    }))
    .into_response()
}
