use std::env;

pub struct Config {
    pub db_url: String
}

impl Default for Config {
    fn default() -> Self {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| panic!("Please set the DATABASE_URL environment variable Bhrata Shree"));

        Self {
            db_url
        }
    }
}
