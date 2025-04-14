use axum::{routing::get, Extension, Router};
use config::{get_config, state::AppState};
use routes::health::get_health;
use tokio::net::TcpListener;

mod config;
mod handlers;
mod models;
mod routes;
mod types;
mod error;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Unable to access .env file");

    let config = get_config();

    let app_state = AppState::init(&config.mongodb_url, "time_capsule").await;

    println!("Connected to Database 👍");

    let listener = TcpListener::bind(config.server_address).await.unwrap();

    let router = Router::new()
        .route("/health", get(get_health))
        .nest("/capsule", routes::capsule::capsule_routes())
        .layer(Extension(app_state));

    println!("Server started on: {} 🚀", listener.local_addr().unwrap());

    axum::serve(listener, router)
        .await
        .expect("Error serving application");
}
