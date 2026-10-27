use std::{fs, path::Path, str::FromStr};

use axum_sqlite_starter::{app, config::Config, migrate};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,tower_http=info".into()))
        .init();

    let config = Config::from_env().expect("invalid configuration");

    let options = SqliteConnectOptions::from_str(&config.database_url)
        .expect("invalid DATABASE_URL")
        .create_if_missing(true)
        .foreign_keys(true);
    if let Some(parent) = Path::new(options.get_filename()).parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent).expect("cannot create the database directory");
    }
    let db = SqlitePoolOptions::new()
        .connect_with(options)
        .await
        .expect("cannot open the database");
    migrate(&db).await.expect("migrations failed");

    let listener = tokio::net::TcpListener::bind(config.addr).await.expect("cannot bind");
    tracing::info!("listening on http://{}", config.addr);
    axum::serve(listener, app(db))
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await
        .expect("server error");
}
