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

    let (status, details) = match db_doc {
        Ok(Some(doc)) => {
            if doc.hash_sha256 == current_hash {
                // -- hash matches: verify cryptographic signature
                let sig_valid =
                    crypto::verify(&current_hash, &payload.signature, state.verify_key.as_str());
                if sig_valid {
                    (
                        DocumentStatus::Valid,
                        serde_json::json!({
                            "document_id":   doc.id,
                            "original_hash": doc.hash_sha256,
                            "current_hash":  current_hash,
                            "author":        doc.author,
                            "signed_at":     doc.signed_at,
                        }),
                    )
                } else {
                    (
                        DocumentStatus::Tampered,
                        serde_json::json!({
                            "reason": "signature mismatch despite matching hash",
                        }),
                    )
                }
            } else {
                // -- hash differs: file was modified
                (
                    DocumentStatus::Tampered,
                    serde_json::json!({
                        "document_id":   doc.id,
                        "original_hash": doc.hash_sha256,
                        "current_hash":  current_hash,
                        "reason":        "content hash does not match registered document",
                    }),
                )
            }
        }
        Ok(None) => {
            // -- payload exists but not in db
            (
                DocumentStatus::Unregistered,
                serde_json::json!({
                    "reason":        "payload found but document not in registry",
                    "current_hash":  current_hash,
                }),
            )
        }
        Err(e) => (
            DocumentStatus::Invalid,
            serde_json::json!({
                "reason": format!("database error: {}", e),
            }),
        ),
    };

    // -- 4. if tampered: store file in corrupted bucket
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

    // -- 5. write audit log
    let _ = audit_repo::create(
        &state.db,
        CreateAuditLog {
            document_id: details
                .get("document_id")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse().ok()),
            result: status.clone(),
            checked_hash: Some(current_hash),
            details: details.clone(),
        },
    )
    .await;

    info!(status = %status, "verification complete");

    Json(ApiResponse::ok(serde_json::json!({
        "status":  status,
        "details": details,
    })))
    .into_response()
}
