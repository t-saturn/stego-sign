use sea_orm::{Database, DatabaseConnection, DbErr};
use tracing::{error, info};

use super::env::Env;

// -- establishes and returns a sea-orm database connection pool
pub async fn connect(env: &Env) -> Result<DatabaseConnection, DbErr> {
    info!(url = %redact_password(&env.database_url), "connecting to database");

    let conn = Database::connect(&env.database_url).await;

    match &conn {
        Ok(_) => info!("database connection established"),
        Err(e) => error!(error = %e, "failed to connect to database"),
    }

    conn
}

// -- pings the database and returns latency in milliseconds
pub async fn ping(conn: &DatabaseConnection) -> Result<f64, DbErr> {
    use sea_orm::ConnectionTrait;
    use std::time::Instant;

    let start = Instant::now();
    conn.execute_unprepared("SELECT 1").await?;
    let elapsed = start.elapsed().as_secs_f64() * 1000.0;

    Ok(elapsed)
}

// -- replaces the password in a connection url with asterisks for safe logging
fn redact_password(url: &str) -> String {
    if let Some(at_pos) = url.rfind('@') {
        if let Some(slash_pos) = url[..at_pos].rfind(':') {
            let mut redacted = url.to_string();
            redacted.replace_range(slash_pos + 1..at_pos, ":***@");
            return redacted.replace(":***@", "@").replacen("//", "//", 1);
        }
    }
    url.to_string()
}
