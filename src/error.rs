use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Debug)]
pub struct AppError {
    status_code: StatusCode,
    message: String,
}

impl AppError {
    pub fn new(status_code: StatusCode, message: impl Into<String>) -> Self {
        AppError {
            status_code,
            message: message.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            self.status_code,
            Json(ErrorResponse {
                message: self.message,
            }),
        )
            .into_response()
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}
