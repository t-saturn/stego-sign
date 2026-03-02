use axum::{Json, extract::State, response::IntoResponse};
use sea_orm::{ConnectionTrait, QueryResult};
use serde::Serialize;
use tracing::info;

use crate::{AppState, models::response::ApiResponse};

#[derive(Debug, Serialize)]
pub struct SignedDoc {
    pub id: String,
    pub filename: String,
    pub hash_sha256: String,
    pub author: String,
    pub signed_at: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct VerificationEntry {
    pub id: String,
    pub document_id: Option<String>,
    pub filename: Option<String>,
    pub result: String,
    pub checked_hash: Option<String>,
    pub checked_at: String,
}

#[derive(Debug, Serialize)]
pub struct RegistryResponse {
    pub signed: Vec<SignedDoc>,
    pub verifications: Vec<VerificationEntry>,
}

// -- GET /api/v1/registry
pub async fn registry_handler(State(state): State<AppState>) -> impl IntoResponse {
    // -- documentos firmados
    let signed_rows: Vec<QueryResult> = state
        .db
        .query_all(sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            r#"
            SELECT id, filename, hash_sha256, author,
                   signed_at, status::text
            FROM app.documents
            ORDER BY signed_at DESC
            "#
            .to_string(),
        ))
        .await
        .unwrap_or_default();

    let signed: Vec<SignedDoc> = signed_rows
        .into_iter()
        .map(|r| SignedDoc {
            id: r
                .try_get::<uuid::Uuid>("", "id")
                .map(|u| u.to_string())
                .unwrap_or_default(),
            filename: r.try_get("", "filename").unwrap_or_default(),
            hash_sha256: r.try_get("", "hash_sha256").unwrap_or_default(),
            author: r.try_get("", "author").unwrap_or_default(),
            signed_at: r
                .try_get::<chrono::DateTime<chrono::Utc>>("", "signed_at")
                .map(|d| d.to_rfc3339())
                .unwrap_or_default(),
            status: r.try_get("", "status").unwrap_or_default(),
        })
        .collect();

    // -- historial de verificaciones con filename del details
    let verify_rows: Vec<QueryResult> = state
        .db
        .query_all(sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            r#"
            SELECT
                a.id,
                a.document_id,
                a.result::text,
                a.checked_hash,
                a.checked_at,
                a.details->>'filename' AS filename
            FROM app.audit_log a
            WHERE a.action = 'VERIFY'
            ORDER BY a.checked_at DESC
            "#
            .to_string(),
        ))
        .await
        .unwrap_or_default();

    let verifications: Vec<VerificationEntry> = verify_rows
        .into_iter()
        .map(|r| VerificationEntry {
            id: r
                .try_get::<uuid::Uuid>("", "id")
                .map(|u| u.to_string())
                .unwrap_or_default(),
            document_id: r
                .try_get::<uuid::Uuid>("", "document_id")
                .map(|u| u.to_string())
                .ok(),
            filename: r.try_get("", "filename").ok(),
            result: r.try_get("", "result").unwrap_or_default(),
            checked_hash: r.try_get("", "checked_hash").ok(),
            checked_at: r
                .try_get::<chrono::DateTime<chrono::Utc>>("", "checked_at")
                .map(|d| d.to_rfc3339())
                .unwrap_or_default(),
        })
        .collect();

    info!(
        signed = signed.len(),
        verifications = verifications.len(),
        "registry requested"
    );

    Json(ApiResponse::ok(RegistryResponse {
        signed,
        verifications,
    }))
    .into_response()
}
