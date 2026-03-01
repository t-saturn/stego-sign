use axum::Json;
use axum::response::IntoResponse;

use crate::models::response::ApiResponse;
use crate::services::crypto;

// -- GET /api/v1/admin/keygen
// -- generates a new ed25519 keypair, only use in development
pub async fn keygen_handler() -> impl IntoResponse {
    match crypto::generate_keypair() {
        Ok((priv_key, pub_key)) => Json(ApiResponse::ok(serde_json::json!({
            "signing_key": priv_key,
            "verify_key":  pub_key,
            "note":        "add these to your .env and remove this endpoint in production",
        })))
        .into_response(),
        Err(e) => Json(ApiResponse::<()>::err(e.to_string())).into_response(),
    }
}
