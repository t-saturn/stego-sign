use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use super::document::DocumentStatus;

#[derive(Debug, Serialize)]
pub struct AuditLog {
    pub id: Uuid,
    pub document_id: Option<Uuid>,
    pub action: String,
    pub checked_at: DateTime<Utc>,
    pub result: DocumentStatus,
    pub checked_hash: Option<String>,
    pub details: serde_json::Value,
}

#[derive(Debug)]
pub struct CreateAuditLog {
    pub document_id: Option<Uuid>,
    pub action: String, // -- "SIGN" | "VERIFY"
    pub result: DocumentStatus,
    pub checked_hash: Option<String>,
    pub details: serde_json::Value,
}
