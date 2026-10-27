use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{error::AppError, AppState};

#[derive(Serialize, sqlx::FromRow)]
pub struct Note {
    id: i64,
    title: String,
    body: String,
    created_at: String,
    updated_at: String,
}

#[derive(Deserialize)]
pub struct NoteInput {
    title: String,
    #[serde(default)]
    body: String,
}

impl NoteInput {
    fn validate(&self) -> Result<(), AppError> {
        if self.title.trim().is_empty() {
            return Err(AppError::Invalid("title can't be blank".into()));
        }
        Ok(())
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index).post(create))
        .route("/{id}", get(show).put(update).delete(destroy))
}

async fn index(State(state): State<AppState>) -> Result<Json<Vec<Note>>, AppError> {
    let notes = sqlx::query_as::<_, Note>("SELECT * FROM notes ORDER BY id DESC")
        .fetch_all(&state.db)
        .await?;
    Ok(Json(notes))
}

async fn show(State(state): State<AppState>, Path(id): Path<i64>) -> Result<Json<Note>, AppError> {
    let note = sqlx::query_as::<_, Note>("SELECT * FROM notes WHERE id = ?")
        .bind(id)
        .fetch_one(&state.db)
        .await?;
    Ok(Json(note))
}

async fn create(
    State(state): State<AppState>,
    Json(input): Json<NoteInput>,
) -> Result<(StatusCode, Json<Note>), AppError> {
    input.validate()?;
    let note = sqlx::query_as::<_, Note>(
        "INSERT INTO notes (title, body) VALUES (?, ?) RETURNING *",
    )
    .bind(&input.title)
    .bind(&input.body)
    .fetch_one(&state.db)
    .await?;
    Ok((StatusCode::CREATED, Json(note)))
}

async fn update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<NoteInput>,
) -> Result<Json<Note>, AppError> {
    input.validate()?;
    let note = sqlx::query_as::<_, Note>(
        "UPDATE notes SET title = ?, body = ?, updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now') \
         WHERE id = ? RETURNING *",
    )
    .bind(&input.title)
    .bind(&input.body)
    .bind(id)
    .fetch_one(&state.db)
    .await?;
    Ok(Json(note))
}

async fn destroy(State(state): State<AppState>, Path(id): Path<i64>) -> Result<StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM notes WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}
