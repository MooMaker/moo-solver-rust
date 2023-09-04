use axum::{response::IntoResponse, http::StatusCode, Json};

pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json(true))
}
