use axum::{extract::State, routing::get, Json, Router};
use serde_json::{json, Value};

use crate::{error::AppError, AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/up", get(up))
}

/// Health check: 200 only when the database answers, so a green probe means the app works.
async fn up(State(state): State<AppState>) -> Result<Json<Value>, AppError> {
    sqlx::query("SELECT 1").execute(&state.db).await?;
    Ok(Json(json!({ "status": "ok" })))
}
