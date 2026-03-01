use axum::{
    Json,
    extract::{Multipart, State},
    response::IntoResponse,
};
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    AppState,
    models::{document::CreateDocument, response::ApiResponse},
    repositories::document as doc_repo,
    services::{crypto, stego, storage},
};

// -- POST /api/v1/sign
// -- receives a file + author, embeds stego payload, stores in minio and db
pub async fn sign_handler(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut file_bytes = None;
    let mut filename = String::from("unknown");
    let mut author = String::from("anonymous");

    // -- extract multipart fields
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
        None => return Json(ApiResponse::<()>::err("no file provided")).into_response(),
    };

    info!(filename = %filename, author = %author, "sign request received");

    // -- 1. compute sha256 of original file
    let hash = crypto::sha256(&bytes);

    // -- 2. sign the hash
    let signature = match crypto::sign(&hash, state.signing_key.as_str()) {
        Ok(s) => s,
        Err(e) => {
            error!(error = %e, "failed to sign document");
            return Json(ApiResponse::<()>::err("signing failed")).into_response();
        }
    };

    // -- 3. embed stego payload into the file
    let document_id = Uuid::new_v4();
    let signed_bytes =
        match stego::embed(&bytes, &filename, document_id, &hash, &signature, &author) {
            Ok(b) => b,
            Err(e) => {
                error!(error = %e, "stego embed failed");
                return Json(ApiResponse::<()>::err("embed failed")).into_response();
            }
        };

    // -- 4. upload original to uploads bucket
    let upload_key = format!("{}/{}", document_id, filename);
    if let Err(e) = storage::upload(
        &state.storage,
        storage::BUCKET_UPLOADS,
        &upload_key,
        bytes,
        "application/octet-stream",
    )
    .await
    {
        error!(error = %e, "upload to minio failed");
        return Json(ApiResponse::<()>::err("storage upload failed")).into_response();
    }

    // -- 5. upload signed file to signatures bucket
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
        return Json(ApiResponse::<()>::err("storage upload failed")).into_response();
    }

    // -- 6. register object in files.objects and document in app.documents
    let object_id = Uuid::new_v4(); // -- in full impl: insert into files.objects first
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
            return Json(ApiResponse::<()>::err("database error")).into_response();
        }
    };

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
