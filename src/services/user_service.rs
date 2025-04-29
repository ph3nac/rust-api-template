use std::sync::Arc;

use uuid::Uuid;

use crate::{db::user_repo::UserRepo, models::user::User};

pub struct UserService {
    repo: Arc<dyn UserRepo>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepo>) -> Self {
        Self { repo }
    }

    pub async fn register(&self, name: String) -> anyhow::Result<User> {
        self.repo.create(name).await
    }
    pub async fn get(&self, id: Uuid) -> anyhow::Result<Option<User>> {
        self.repo.find(id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use uuid::Uuid;

    struct DummyRepo;
    #[async_trait]
    impl UserRepo for DummyRepo {
        async fn create(&self, name: String) -> anyhow::Result<User> {
            Ok(User {
                id: Uuid::new_v4(),
                name,
            })
        }
        async fn find(&self, id: Uuid) -> anyhow::Result<Option<User>> {
            Ok(Some(User {
                id,
                name: "TestUser".into(),
            }))
        }
    }

    #[tokio::test]
    async fn service_register_and_get() {
        let repo = Arc::new(DummyRepo);
        let service = UserService::new(repo.clone());
        let name = "Bob".to_string();
        let user = service.register(name.clone()).await.unwrap();
        assert_eq!(user.name, name);

        let fetched = service.get(user.id).await.unwrap().unwrap();
        assert_eq!(fetched.id, user.id);
    }
}
