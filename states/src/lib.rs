use std::sync::Arc;

use configs::Settings;
use db::user_repo::{PgUserRepo, UserRepo};
use sqlx::postgres::PgPoolOptions;

#[derive(Clone)]
pub struct AppState {
    pub user_repo: Arc<dyn UserRepo>,
}

impl AppState {
    pub async fn new(cfg: &Settings) -> anyhow::Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&cfg.database_url)
            .await?;
        let user_repo = Arc::new(PgUserRepo::new(pool));
        Ok(Self { user_repo })
    }
}
