use chrono::{DateTime, Utc};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCapsuleRequest {
    #[validate(length(min = 5, message = "name should be atleast 5 characters long"))]
    pub name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 5, message = "title should be atleast 5 characters long"))]
    pub title: String,

    #[validate(length(min = 5, message = "message should be atleast 5 characters long"))]
    pub message: String,

    pub unlock_at: DateTime<Utc>,
}
