use std::env;

use dotenvy::dotenv;

pub struct Config {
    pub db_url: String
}

impl Default for Config {
    fn default() -> Self {
        dotenv().ok();
        
        let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| panic!("Please set the DATABASE_URL environment variable bhrata shree"));

        Self {
            db_url
        }
    }
}
