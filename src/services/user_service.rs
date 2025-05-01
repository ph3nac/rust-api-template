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
    use crate::db::user_repo::MockUserRepo;

    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn service_register_and_get() {
        let mut mock_user_repo = MockUserRepo::new();
        mock_user_repo.expect_create().returning(|name| {
            Ok(User {
                id: Uuid::new_v4(),
                name,
            })
        });

        mock_user_repo.expect_find().returning(|id| {
            Ok(Some(User {
                id,
                name: "TestUser".into(),
            }))
        });

        let repo = Arc::new(mock_user_repo);
        let service = UserService::new(repo.clone());
        let name = "Bob".to_string();
        let user = service.register(name.clone()).await.unwrap();
        assert_eq!(user.name, name);

        let fetched = service.get(user.id).await.unwrap().unwrap();
        assert_eq!(fetched.id, user.id);
    }
}
