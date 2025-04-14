use axum::{routing::get, Router};

use crate::handlers::capsule::{create_capsule, get_capsules, get_capsules_by_public_id};

pub fn capsule_routes() -> Router {
    Router::new()
        .route("/", get(get_capsules).post(create_capsule))
        .route("/{public_id}", get(get_capsules_by_public_id))
}
