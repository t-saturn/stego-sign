use axum::http::StatusCode;
use axum::{
    Json,
    extract::{Multipart, State},
    response::IntoResponse,
};
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    AppState,
    models::{
        audit_log::CreateAuditLog,
        document::{CreateDocument, DocumentStatus},
        response::{ApiError, ApiResponse},
    },
    repositories::{audit_log as audit_repo, document as doc_repo, object as obj_repo},
    services::{crypto, stego, storage},
};

// -- POST /api/v1/sign
pub async fn sign_handler(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut file_bytes = None;
    let mut filename = String::from("unknown");
    let mut author = String::from("anonymous");

    while let Ok(Some(field)) = multipart.next_field().await {
        match field.name() {
            Some("file") => {
                filename = field.file_name().unwrap_or("file").to_string();
                file_bytes = field.bytes().await.ok();
            }
            Some("author") => {
                author = field.text().await.unwrap_or_default();
            }
            _ => {}
        }
    }

    let bytes = match file_bytes {
        Some(b) => b,
        None => {
            return ApiError {
                status: StatusCode::BAD_REQUEST,
                message: "no file provided".to_string(),
            }
            .into_response();
        }
    };

    info!(filename = %filename, author = %author, "sign request received");

    let file_size = bytes.len() as i64;

    // -- 1. sha256 del original
    let hash = crypto::sha256(&bytes);

    // -- 2. firma el hash con Ed25519
    let signature = match crypto::sign(&hash, state.signing_key.as_str()) {
        Ok(s) => s,
        Err(e) => {
            return ApiError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("signing failed: {}", e),
            }
            .into_response();
        }
    };

    // -- 3. embebe payload esteganográfico
    let document_id = Uuid::new_v4();
    let signed_bytes =
        match stego::embed(&bytes, &filename, document_id, &hash, &signature, &author) {
            Ok(b) => b,
            Err(e) => {
                error!(error = %e, "stego embed failed");
                return ApiError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "embed failed".to_string(),
                }
                .into_response();
            }
        };

    // -- 4. sube original al bucket uploads
    let upload_key = format!("{}/{}", document_id, filename);
    if let Err(e) = storage::upload(
        &state.storage,
        storage::BUCKET_UPLOADS,
        &upload_key,
        bytes.clone(),
        "application/octet-stream",
    )
    .await
    {
        error!(error = %e, "upload original to minio failed");
        return ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "storage upload failed".to_string(),
        }
        .into_response();
    }

    // -- 5. sube archivo firmado al bucket signatures
    let signed_key = format!("{}/signed_{}", document_id, filename);
    if let Err(e) = storage::upload(
        &state.storage,
        storage::BUCKET_SIGNATURES,
        &signed_key,
        signed_bytes,
        "application/octet-stream",
    )
    .await
    {
        error!(error = %e, "upload signed file failed");
        return ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "storage upload failed".to_string(),
        }
        .into_response();
    }

    // -- 6. registra original en files.objects
    let object_id = match obj_repo::register(
        &state.db,
        obj_repo::CreateObject {
            bucket_name: storage::BUCKET_UPLOADS.to_string(),
            object_key: upload_key.clone(),
            filename: filename.clone(),
            content_type: "application/octet-stream".to_string(),
            size_bytes: file_size,
        },
    )
    .await
    {
        Ok(id) => id,
        Err(e) => {
            error!(error = %e, "failed to register original in files.objects");
            return ApiError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "database error".to_string(),
            }
            .into_response();
        }
    };

    // -- 7. registra archivo firmado en files.objects
    let signed_size = {
        // -- aproximación: el firmado es ligeramente mayor
        file_size
    };
    let _ = obj_repo::register(
        &state.db,
        obj_repo::CreateObject {
            bucket_name: storage::BUCKET_SIGNATURES.to_string(),
            object_key: signed_key.clone(),
            filename: format!("signed_{}", filename),
            content_type: "application/octet-stream".to_string(),
            size_bytes: signed_size,
        },
    )
    .await;

    // -- 8. registra documento en app.documents
    let doc_id = match doc_repo::create(
        &state.db,
        CreateDocument {
            filename: filename.clone(),
            hash_sha256: hash.clone(),
            signature: signature.clone(),
            author: author.clone(),
            object_id,
            metadata: Some(serde_json::json!({
                "upload_key": upload_key,
                "signed_key": signed_key,
            })),
        },
    )
    .await
    {
        Ok(id) => id,
        Err(e) => {
            error!(error = %e, "db insert failed");
            return ApiError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "database error".to_string(),
            }
            .into_response();
        }
    };

    // -- 9. registra en audit_log con action SIGN
    let _ = audit_repo::create(
        &state.db,
        CreateAuditLog {
            document_id: Some(doc_id),
            action: "SIGN".to_string(),
            result: DocumentStatus::Valid,
            checked_hash: Some(hash.clone()),
            details: serde_json::json!({
                "filename":   filename,
                "author":     author,
                "upload_key": upload_key,
                "signed_key": signed_key,
            }),
        },
    )
    .await;

    info!(document_id = %doc_id, "document signed and stored");

    Json(ApiResponse::ok(serde_json::json!({
        "document_id": doc_id,
        "filename":    filename,
        "hash":        hash,
        "author":      author,
        "signed_key":  signed_key,
    })))
    .into_response()
}
