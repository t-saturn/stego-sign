pub fn api_base_url() -> String {
    #[cfg(feature = "ssr")]
    {
        std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://server:4000".to_string())
    }
    #[cfg(not(feature = "ssr"))]
    {
        web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.query_selector("meta[name='api-base-url']").ok().flatten())
            .and_then(|el| el.get_attribute("content"))
            .unwrap_or_else(|| "http://localhost:55547".to_string())
    }
}

pub fn app_env() -> String {
    option_env!("APP_ENV").unwrap_or("development").to_string()
}

pub fn is_development() -> bool {
    app_env() == "development"
}
