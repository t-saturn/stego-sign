// -- api base url, reads from env at compile time or falls back to default
pub fn api_base_url() -> String {
    option_env!("API_BASE_URL")
        .unwrap_or("http://localhost:4000")
        .to_string()
}

pub fn is_development() -> bool {
    option_env!("APP_ENV").unwrap_or("development") == "development"
}
