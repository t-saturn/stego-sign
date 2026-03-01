use tracing_appender::rolling;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

// -- initializes tracing with two outputs: terminal (pretty) and rolling file (json)
pub fn init() {
    // -- create logs/ directory and rolling daily log file
    let file_appender = rolling::daily("logs", "stego-server.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // -- leak the guard so the file writer lives for the entire process lifetime
    Box::leak(Box::new(guard));

    // -- filter: respect RUST_LOG env var, default to info
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // -- terminal layer: human-readable, colored
    let terminal_layer = fmt::layer()
        .pretty()
        .with_target(true)
        .with_thread_ids(false)
        .with_file(false);

    // -- file layer: json format for structured log parsing
    let file_layer = fmt::layer()
        .json()
        .with_writer(non_blocking)
        .with_target(true)
        .with_current_span(true);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(terminal_layer)
        .with(file_layer)
        .with(tracing_error::ErrorLayer::default())
        .init();
}
