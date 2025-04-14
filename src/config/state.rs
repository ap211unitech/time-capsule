use mongodb::{options::ClientOptions, Client, Database};

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
}

impl AppState {
    pub async fn init(mongodb_url: &str, db_name: &str) -> Self {
        let client_options = ClientOptions::parse(mongodb_url).await.unwrap();
        let client = Client::with_options(client_options).unwrap();

        AppState {
            db: client.database(db_name),
        }
    }
}
