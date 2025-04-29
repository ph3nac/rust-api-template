use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;

use crate::{
    config::Settings,
    db::user_repo::{PgUserRepo, UserRepo},
};

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
