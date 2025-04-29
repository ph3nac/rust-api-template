use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub database_url: String,
    pub server_address: String,
}

pub fn load() -> anyhow::Result<Settings> {
    // dotenv().expect("Failed to read .env file");
    dotenv().ok();
    Ok(Settings {
        database_url: dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        server_address: dotenv::var("SERVER_ADDRESS").expect("SERVER_ADDRESS must be set"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn load_with_env() {
        unsafe {
            env::set_var("DATABASE_URL", "postgres://user:pass@localhost/db");
            env::set_var("SERVER_ADDRESS", "0.0.0.0:8080");
        }
        let cfg = load().unwrap();
        assert_eq!(cfg.database_url, "postgres://user:pass@localhost/db");
        assert_eq!(cfg.server_address, "0.0.0.0:8080");
    }
}
