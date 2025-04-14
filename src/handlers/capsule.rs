use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use validator::Validate;

use crate::{
    config::state::AppState,
    error::AppError,
    models::capsule::{CapsuleModel, CapsuleTrait},
    types::{request::CreateCapsuleRequest, responses::CreateCapsuleResponse},
};

pub async fn get_capsules(
    Extension(app_state): Extension<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let capsules = CapsuleModel::get_capsules(&app_state.db)
        .await
        .map_err(|err| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database Error: {}", err),
            )
        })?;

    Ok((StatusCode::OK, Json(capsules)))
}

pub async fn create_capsule(
    Extension(app_state): Extension<AppState>,
    Json(payload): Json<CreateCapsuleRequest>,
) -> Result<impl IntoResponse, AppError> {
    if let Err(errors) = payload.validate() {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            format!("Validation error: {}", errors),
        ));
    }

    let capsule = CapsuleModel::create_capsule(
        &app_state.db,
        &payload.name,
        &payload.email,
        &payload.title,
        &payload.message,
        &payload.unlock_at,
    )
    .await
    .map_err(|err| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("DB insert failed: {}", err),
        )
    })?;

    Ok((
        StatusCode::CREATED,
        Json(CreateCapsuleResponse {
            _id: capsule.inserted_id.as_object_id().unwrap().to_hex(),
        }),
    ))
}
