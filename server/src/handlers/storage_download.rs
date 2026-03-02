use axum::{
    Json,
    body::Body,
    extract::{Path, State},
    http::{StatusCode, header},
    response::IntoResponse,
};
use tracing::info;

use crate::{AppState, models::response::ApiResponse};

// -- GET /api/v1/storage/{bucket}/{*key}
pub async fn storage_download_handler(
    State(state): State<AppState>,
    Path((bucket, key)): Path<(String, String)>,
) -> impl IntoResponse {
    info!(bucket = %bucket, key = %key, "storage download requested");

    let allowed = ["uploads", "signatures", "corrupted"];
    if !allowed.contains(&bucket.as_str()) {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<()>::err("invalid bucket")),
        )
            .into_response();
    }

    match crate::services::storage::download_storage(&state.storage, &bucket, &key).await {
        Ok((bytes, content_type)) => {
            let filename = key.split('/').last().unwrap_or("file");
            (
                StatusCode::OK,
                [
                    (header::CONTENT_TYPE, content_type),
                    (
                        header::CONTENT_DISPOSITION,
                        format!("attachment; filename=\"{}\"", filename),
                    ),
                ],
                Body::from(bytes),
            )
                .into_response()
        }
        Err(e) => (StatusCode::NOT_FOUND, Json(ApiResponse::<()>::err(&e))).into_response(),
    }
}
