#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    // -- inicializa tracing para ver logs
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use stego_app::app::*;
    use stego_app::App;

    let conf = get_configuration(None).unwrap();
    let options = conf.leptos_options;
    let addr = options.site_addr;
    let routes = generate_route_list(App);

    let api_url =
        std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://server:4000".to_string());

    let client = reqwest::Client::new();

    // -- proxy: reenvía /api/* al server interno
    let app = Router::new()
        .route(
            "/api/{*path}",
            axum::routing::any({
                // <-- primero
                let client = client.clone();
                let api_url = api_url.clone();
                move |req| proxy_handler(req, client.clone(), api_url.clone())
            }),
        )
        .leptos_routes(&options, routes, {
            // <-- después
            let options = options.clone();
            move || shell(options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(feature = "ssr")]
async fn proxy_handler(
    req: axum::extract::Request,
    client: reqwest::Client,
    api_url: String,
) -> impl axum::response::IntoResponse {
    use axum::http::{HeaderValue, StatusCode};

    let path = req
        .uri()
        .path_and_query()
        .map(|p| p.as_str())
        .unwrap_or("/");

    let target = format!("{}{}", api_url, path);
    tracing::info!("proxy → {}", target);

    let method = reqwest::Method::from_bytes(req.method().as_str().as_bytes()).unwrap();
    let headers = req.headers().clone();
    let body = axum::body::to_bytes(req.into_body(), usize::MAX)
        .await
        .unwrap_or_default();

    let mut proxy_req = client.request(method, &target).body(body);
    for (key, val) in headers.iter() {
        // -- no reenviar estos headers, causan problemas
        if key == axum::http::header::HOST
            || key == axum::http::header::ORIGIN
            || key.as_str() == "sec-fetch-site"
            || key.as_str() == "sec-fetch-mode"
            || key.as_str() == "sec-fetch-dest"
        {
            continue;
        }
        proxy_req = proxy_req.header(key, val);
    }

    match proxy_req.send().await {
        Ok(resp) => {
            tracing::info!("proxy response: {}", resp.status());
            let status =
                StatusCode::from_u16(resp.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
            let resp_headers = resp.headers().clone();
            let body = resp.bytes().await.unwrap_or_default();

            let mut response = axum::response::Response::new(axum::body::Body::from(body));
            *response.status_mut() = status;
            *response.headers_mut() = resp_headers;
            // -- agrega CORS para el browser
            response.headers_mut().insert(
                axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                HeaderValue::from_static("*"),
            );
            response
        }
        Err(e) => {
            tracing::error!("proxy error: {}", e);
            axum::response::Response::builder()
                .status(StatusCode::BAD_GATEWAY)
                .body(axum::body::Body::empty())
                .unwrap()
        }
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
