use axum::{
    Json,
    extract::{Multipart, State},
    response::IntoResponse,
};
use tracing::{info, warn};

use crate::{
    AppState,
    models::{audit_log::CreateAuditLog, document::DocumentStatus, response::ApiResponse},
    repositories::{audit_log as audit_repo, document as doc_repo},
    services::{crypto, stego, storage},
};

// -- POST /api/v1/verify
// -- extracts stego payload, cross-checks with db, returns forensic analysis
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

    // -- 1. compute current hash
    // let current_hash = crypto::sha256(&bytes);

    // -- strip stego block before hashing to get original content hash
    let stripped = stego::strip(&bytes);
    let current_hash = crypto::sha256(&stripped);

    // -- 2. extract stego payload
    let payload = match stego::extract(&filename, &bytes) {
        Ok(p) => p,
        Err(_) => {
            warn!(filename = %filename, "no stego payload found");
            let _ = audit_repo::create(
                &state.db,
                CreateAuditLog {
                    document_id: None,
                    result: DocumentStatus::Invalid,
                    checked_hash: Some(current_hash.clone()),
                    details: serde_json::json!({ "reason": "no payload found" }),
                },
            )
            .await;
            return Json(ApiResponse::ok(serde_json::json!({
                "status":       "INVALID",
                "current_hash": current_hash,
                "reason":       "no steganographic payload found in file",
            })))
            .into_response();
        }
    };

    // -- 3. cross-check with db by document_id from payload
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
                            "reason": "signature mismatch despite matching hash",
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

    // -- si tampered: guarda en corrupted bucket (igual que antes)
    if status == DocumentStatus::Tampered {
        let key = format!("corrupted/{}", filename);
        let _ = storage::upload(
            &state.storage,
            storage::BUCKET_CORRUPTED,
            &key,
            bytes,
            "application/octet-stream",
        )
        .await;
        warn!(filename = %filename, "tampered file stored in corrupted bucket");
    }

    // -- audit log (igual que antes)
    let _ = audit_repo::create(
        &state.db,
        CreateAuditLog {
            document_id: details
                .get("document_id")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse().ok()),
            result: status.clone(),
            checked_hash: Some(current_hash.clone()),
            details: details.clone(),
        },
    )
    .await;

    info!(status = %status, "verification complete");

    // -- respuesta con booleanos explícitos
    Json(ApiResponse::ok(serde_json::json!({
        "status":          status,
        "hash_match":      hash_match,
        "signature_valid": signature_valid,
        "registered":      registered,
        "document_id":     details.get("document_id"),
        "filename":        details.get("filename"),
        "author":          details.get("author"),
        "signed_at":       details.get("signed_at"),
        "hash":            details.get("hash"),
    })))
    .into_response()
}
