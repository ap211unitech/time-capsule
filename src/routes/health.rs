use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    message: String,
}

pub async fn get_health() -> impl IntoResponse {
    let response = HealthResponse {
        message: "Server is too healthy!".to_owned(),
    };

    (StatusCode::OK, Json(response))
}
