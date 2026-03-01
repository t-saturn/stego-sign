use tower_http::cors::{Any, CorsLayer};

// -- builds cors layer allowing all origins, methods and headers
// -- in production: replace Any with specific origins
pub fn layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}
