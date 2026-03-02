use crate::shared::config::api_base_url;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Document {
    pub status: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
}

#[derive(Debug, Clone, Default)]
pub struct Stats {
    pub total: usize,
    pub tampered: usize,
}

pub async fn fetch_stats() -> Result<Stats, String> {
    let resp = gloo_net::http::Request::get(&format!("{}/api/v1/documents", api_base_url()))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let parsed: ApiResponse<Vec<Document>> = resp.json().await.map_err(|e| e.to_string())?;

    let docs = parsed.data.unwrap_or_default();
    let tampered = docs.iter().filter(|d| d.status == "TAMPERED").count();

    Ok(Stats {
        total: docs.len(),
        tampered,
    })
}
