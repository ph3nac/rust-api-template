use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user::User;

#[async_trait]
pub trait UserRepo: Send + Sync + 'static {
    async fn create(&self, name: String) -> anyhow::Result<User>;
    async fn find(&self, id: Uuid) -> anyhow::Result<Option<User>>;
}

pub struct PgUserRepo {
    pool: PgPool,
}

impl PgUserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
#[async_trait]
impl UserRepo for PgUserRepo {
    async fn create(&self, name: String) -> anyhow::Result<User> {
        let query = format!(
            "INSERT INTO users (name) VALUES ('{}') RETURNING id, name",
            name
        );
        let user = sqlx::query_as::<_, User>(&query)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }
    async fn find(&self, id: Uuid) -> anyhow::Result<Option<User>> {
        let query = "SELECT id, name FROM users WHERE id = $1";
        let user = sqlx::query_as::<_, User>(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }
}
