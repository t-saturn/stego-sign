use axum::{
    Json,
    extract::{Multipart, State},
    response::IntoResponse,
};
use sea_orm::ConnectionTrait;
use tracing::{info, warn};

use crate::{
    AppState,
    models::{audit_log::CreateAuditLog, document::DocumentStatus, response::ApiResponse},
    repositories::{audit_log as audit_repo, document as doc_repo, object as obj_repo},
    services::{crypto, stego, storage},
};

pub async fn verify_handler(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut file_bytes = None;
    let mut filename = String::from("unknown");

    while let Ok(Some(field)) = multipart.next_field().await {
        if field.name() == Some("file") {
            filename = field.file_name().unwrap_or("file").to_string();
            file_bytes = field.bytes().await.ok();
        }
    }

    let bytes = match file_bytes {
        Some(b) => b,
        None => return Json(ApiResponse::<()>::err("no file provided")).into_response(),
    };

    info!(filename = %filename, "verify request received");

    let size_bytes = bytes.len() as i64;

    // -- 1. strip payload antes de hashear
    let stripped = stego::strip(&bytes);
    let current_hash = crypto::sha256(&stripped);

    // -- 2. sube el archivo al bucket uploads siempre
    let upload_key = format!("verify/{}/{}", uuid::Uuid::new_v4(), filename);
    let _ = storage::upload(
        &state.storage,
        storage::BUCKET_UPLOADS,
        &upload_key,
        bytes.clone(),
        "application/octet-stream",
    )
    .await;

    let _ = obj_repo::register(
        &state.db,
        obj_repo::CreateObject {
            bucket_name: storage::BUCKET_UPLOADS.to_string(),
            object_key: upload_key.clone(),
            filename: filename.clone(),
            content_type: "application/octet-stream".to_string(),
            size_bytes,
        },
    )
    .await;

    // -- 3. extrae payload esteganográfico
    let payload = match stego::extract(&filename, &bytes) {
        Ok(p) => p,
        Err(_) => {
            warn!(filename = %filename, "no stego payload found");

            let _ = audit_repo::create(
                &state.db,
                CreateAuditLog {
                    document_id: None,
                    action: "VERIFY".to_string(),
                    result: DocumentStatus::Invalid,
                    checked_hash: Some(current_hash.clone()),
                    details: serde_json::json!({
                        "reason":   "no payload found",
                        "filename": filename,
                    }),
                },
            )
            .await;

            return Json(ApiResponse::ok(serde_json::json!({
                "status":          "INVALID",
                "hash_match":      false,
                "signature_valid": false,
                "registered":      false,
                "filename":        filename,
                "current_hash":    current_hash,
            })))
            .into_response();
        }
    };

    // -- 4. cross-check con db
    let db_doc = doc_repo::find_by_hash(&state.db, &payload.original_hash).await;

    let (status, hash_match, signature_valid, registered, details) = match db_doc {
        Ok(Some(doc)) => {
            if doc.hash_sha256 == current_hash {
                let sig_valid =
                    crypto::verify(&current_hash, &payload.signature, state.verify_key.as_str());
                if sig_valid {
                    (
                        DocumentStatus::Valid,
                        true,
                        true,
                        true,
                        serde_json::json!({
                            "document_id": doc.id,
                            "filename":    doc.filename,
                            "author":      doc.author,
                            "signed_at":   doc.signed_at,
                            "hash":        doc.hash_sha256,
                        }),
                    )
                } else {
                    (
                        DocumentStatus::Tampered,
                        true,
                        false,
                        true,
                        serde_json::json!({
                            "document_id": doc.id,
                            "filename":    doc.filename,
                            "reason":      "signature mismatch despite matching hash",
                        }),
                    )
                }
            } else {
                (
                    DocumentStatus::Tampered,
                    false,
                    false,
                    true,
                    serde_json::json!({
                        "document_id":   doc.id,
                        "filename":      doc.filename,
                        "original_hash": doc.hash_sha256,
                        "current_hash":  current_hash,
                        "reason":        "content hash does not match registered document",
                    }),
                )
            }
        }
        Ok(None) => (
            DocumentStatus::Unregistered,
            true,
            true,
            false,
            serde_json::json!({
                "reason":       "payload found but document not in registry",
                "filename":     filename,
                "current_hash": current_hash,
            }),
        ),
        Err(e) => (
            DocumentStatus::Invalid,
            false,
            false,
            false,
            serde_json::json!({ "reason": format!("database error: {}", e) }),
        ),
    };

    // -- 5. si tampered: actualiza status en db + sube a corrupted
    if status == DocumentStatus::Tampered {
        // -- actualiza el documento en app.documents
        if let Some(doc_id_str) = details.get("document_id").and_then(|v| v.as_str()) {
            if let Ok(doc_uuid) = doc_id_str.parse::<uuid::Uuid>() {
                let _ = state.db.execute(
                    sea_orm::Statement::from_sql_and_values(
                        sea_orm::DatabaseBackend::Postgres,
                        "UPDATE app.documents SET status = 'TAMPERED'::app.document_status WHERE id = $1",
                        [doc_uuid.into()],
                    )
                ).await;
            }
        }

        // -- sube a bucket corrupted
        let corrupted_key = format!("corrupted/{}/{}", uuid::Uuid::new_v4(), filename);
        let _ = storage::upload(
            &state.storage,
            storage::BUCKET_CORRUPTED,
            &corrupted_key,
            bytes.clone(),
            "application/octet-stream",
        )
        .await;

        let _ = obj_repo::register(
            &state.db,
            obj_repo::CreateObject {
                bucket_name: storage::BUCKET_CORRUPTED.to_string(),
                object_key: corrupted_key,
                filename: format!("corrupted_{}", filename),
                content_type: "application/octet-stream".to_string(),
                size_bytes,
            },
        )
        .await;

        warn!(filename = %filename, "tampered file stored in corrupted bucket");
    }

    // -- 6. audit log — incluye upload_key para poder descargar después
    let _ = audit_repo::create(
        &state.db,
        CreateAuditLog {
            document_id: details
                .get("document_id")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse().ok()),
            action: "VERIFY".to_string(),
            result: status.clone(),
            checked_hash: Some(current_hash.clone()),
            details: {
                let mut d = details.clone();
                d["upload_key"] = serde_json::json!(upload_key);
                d
            },
        },
    )
    .await;

    info!(status = %status, "verification complete");

    Json(ApiResponse::ok(serde_json::json!({
        "status":          status,
        "hash_match":      hash_match,
        "signature_valid": signature_valid,
        "registered":      registered,
        "document_id":     details.get("document_id").and_then(|v| v.as_str()),
        "filename":        details.get("filename").and_then(|v| v.as_str()),
        "author":          details.get("author").and_then(|v| v.as_str()),
        "signed_at":       details.get("signed_at"),
        "hash":            details.get("hash").and_then(|v| v.as_str()),
    })))
    .into_response()
}
