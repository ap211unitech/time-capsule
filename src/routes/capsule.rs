use axum::{routing::get, Router};

use crate::handlers::capsule::{create_capsule, get_capsules};

pub fn capsule_routes() -> Router {
    Router::new().route("/", get(get_capsules).post(create_capsule))
}
