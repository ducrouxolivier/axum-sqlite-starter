pub mod config;
pub mod error;
pub mod routes;

use axum::Router;
use sqlx::SqlitePool;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

/// Applies the migrations in `./migrations`. Called on boot, so a fresh checkout needs no setup step.
pub async fn migrate(db: &SqlitePool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!().run(db).await
}

pub fn app(db: SqlitePool) -> Router {
    Router::new()
        .merge(routes::health::router())
        .nest("/api/notes", routes::notes::router())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(AppState { db })
}
