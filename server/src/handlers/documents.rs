use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use tracing::info;
use uuid::Uuid;

use crate::{
    AppState,
    models::response::ApiResponse,
    repositories::{audit_log as audit_repo, document as doc_repo},
};

// -- GET /api/v1/documents
pub async fn list_handler(State(state): State<AppState>) -> impl IntoResponse {
    info!("list documents requested");
    match doc_repo::list(&state.db, 100).await {
        Ok(docs) => Json(ApiResponse::ok(docs)).into_response(),
        Err(e) => Json(ApiResponse::<()>::err(e.to_string())).into_response(),
    }
}

// -- GET /api/v1/documents/:id
pub async fn get_handler(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    info!(document_id = %id, "get document requested");
    match doc_repo::find_by_id(&state.db, id).await {
        Ok(Some(doc)) => Json(ApiResponse::ok(doc)).into_response(),
        Ok(None) => Json(ApiResponse::<()>::err("document not found")).into_response(),
        Err(e) => Json(ApiResponse::<()>::err(e.to_string())).into_response(),
    }
}

// -- GET /api/v1/documents/:id/audit
pub async fn audit_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    info!(document_id = %id, "audit log requested");
    match audit_repo::list_by_document(&state.db, id).await {
        Ok(logs) => Json(ApiResponse::ok(logs)).into_response(),
        Err(e) => Json(ApiResponse::<()>::err(e.to_string())).into_response(),
    }
}
