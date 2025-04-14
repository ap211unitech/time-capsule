pub mod state;

pub struct Config {
    pub server_address: String,
    pub mongodb_url: String,
}

pub fn get_config() -> Config {
    Config {
        server_address: std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:8000".to_string()),
        mongodb_url: std::env::var("MONGODB_URL").unwrap_or("".to_string()),
    }
}
