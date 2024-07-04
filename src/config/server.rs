use std::env;

pub struct Config {
    pub host: String,
    pub port: String,
}

pub fn load_config() -> Config {
    dotenv::dotenv().ok();

    Config {
        host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        port: env::var("PORT").unwrap_or_else(|_| "8080".to_string()),
    }
}
