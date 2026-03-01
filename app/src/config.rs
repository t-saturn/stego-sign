// -- api base url, reads from env at build time or falls back to default
// -- set API_BASE_URL in .env before running cargo leptos watch

pub fn api_base_url() -> String {
    option_env!("API_BASE_URL")
        .unwrap_or("http://localhost:4000")
        .to_string()
}

pub fn app_env() -> String {
    option_env!("APP_ENV").unwrap_or("development").to_string()
}

pub fn is_development() -> bool {
    app_env() == "development"
}
