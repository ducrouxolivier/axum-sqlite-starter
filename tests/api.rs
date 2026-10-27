use axum::{
    body::Body,
    http::{header, Method, Request, StatusCode},
    Router,
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use sqlx::sqlite::SqlitePoolOptions;
use tower::ServiceExt;

async fn test_app() -> Router {
    // One connection: every pooled connection to :memory: would otherwise be its own empty database.
    let db = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    axum_sqlite_starter::migrate(&db).await.unwrap();
    axum_sqlite_starter::app(db)
}

async fn call(app: &Router, method: Method, uri: &str, body: Option<Value>) -> (StatusCode, Value) {
    let mut request = Request::builder().method(method).uri(uri);
    let body = match body {
        Some(json) => {
            request = request.header(header::CONTENT_TYPE, "application/json");
            Body::from(json.to_string())
        }
        None => Body::empty(),
    };
    let response = app.clone().oneshot(request.body(body).unwrap()).await.unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let json = if bytes.is_empty() { Value::Null } else { serde_json::from_slice(&bytes).unwrap() };
    (status, json)
}

#[tokio::test]
async fn up_is_200_when_the_database_answers() {
    let app = test_app().await;
    let (status, body) = call(&app, Method::GET, "/up", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "ok");
}

#[tokio::test]
async fn notes_crud_round_trip() {
    let app = test_app().await;

    let (status, created) = call(&app, Method::POST, "/api/notes", Some(json!({ "title": "First", "body": "hello" }))).await;
    assert_eq!(status, StatusCode::CREATED);
    let id = created["id"].as_i64().unwrap();

    let (status, shown) = call(&app, Method::GET, &format!("/api/notes/{id}"), None).await;
    assert_eq!((status, shown["title"].as_str()), (StatusCode::OK, Some("First")));

    let (status, updated) = call(&app, Method::PUT, &format!("/api/notes/{id}"), Some(json!({ "title": "Renamed" }))).await;
    assert_eq!((status, updated["title"].as_str(), updated["body"].as_str()), (StatusCode::OK, Some("Renamed"), Some("")));

    let (_, list) = call(&app, Method::GET, "/api/notes", None).await;
    assert_eq!(list.as_array().unwrap().len(), 1);

    let (status, _) = call(&app, Method::DELETE, &format!("/api/notes/{id}"), None).await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    let (status, _) = call(&app, Method::GET, &format!("/api/notes/{id}"), None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn blank_title_is_rejected_and_nothing_is_stored() {
    let app = test_app().await;
    let (status, body) = call(&app, Method::POST, "/api/notes", Some(json!({ "title": "  " }))).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(body["error"].as_str().unwrap().contains("title"));
    let (_, list) = call(&app, Method::GET, "/api/notes", None).await;
    assert!(list.as_array().unwrap().is_empty());
}

#[tokio::test]
async fn deleting_or_updating_a_missing_note_is_404() {
    let app = test_app().await;
    let (status, _) = call(&app, Method::DELETE, "/api/notes/999", None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let (status, _) = call(&app, Method::PUT, "/api/notes/999", Some(json!({ "title": "x" }))).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}
