use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{models::user::User, services::user_service::UserService, state::AppState};

#[derive(Deserialize)]
struct RegisterPayload {
    name: String,
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<User>, (StatusCode, String)> {
    let service = UserService::new(state.user_repo.clone());
    let user = service
        .register(payload.name)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(user))
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, (StatusCode, String)> {
    let service = UserService::new(state.user_repo.clone());
    match service.get(id).await {
        Ok(Some(u)) => Ok(Json(u)),
        Ok(None) => Err((axum::http::StatusCode::NOT_FOUND, "Not Found".into())),
        Err(e) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(register))
        .route("/:id", get(get_user))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{db::user_repo::MockUserRepo, state::AppState};
    use axum::{
        Json,
        extract::{Path, State},
        http::StatusCode,
    };
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_register() {
        let mut user_repo = Arc::new(MockUserRepo::default());
        Arc::get_mut(&mut user_repo)
            .unwrap()
            .expect_create()
            .returning(|_| {
                Ok(crate::models::user::User {
                    id: uuid::Uuid::new_v4(),
                    name: "Test User".to_string(),
                })
            });

        let state = AppState { user_repo };
        let payload = RegisterPayload {
            name: "Test User".to_string(),
        };

        let result = register(State(state), Json(payload)).await;

        assert!(result.is_ok());
        let user = result.unwrap().0;
        assert_eq!(user.name, "Test User");
    }

    #[tokio::test]
    async fn test_get_user_existing() {
        let mut user_repo = MockUserRepo::new();
        let user = crate::models::user::User {
            id: uuid::Uuid::new_v4(),
            name: "Test User".into(),
        };

        user_repo
            .expect_create()
            .returning(move |name| Ok(User { id: user.id, name }));
        user_repo.expect_find().returning(move |id| {
            Ok(Some(User {
                id,
                name: "Test User".into(),
            }))
        });

        let state = AppState {
            user_repo: Arc::new(user_repo),
        };

        let result = get_user(State(state), Path(user.id)).await;

        assert!(result.is_ok());
        let retrieved_user = result.unwrap().0;
        assert_eq!(retrieved_user.id, user.id);
    }

    #[tokio::test]
    async fn test_get_user_not_found() {
        let mut user_repo = Arc::new(MockUserRepo::default());
        Arc::get_mut(&mut user_repo)
            .unwrap()
            .expect_find()
            .returning(|_| Ok(None));

        let state = AppState { user_repo };
        let non_existent_id = Uuid::new_v4();

        let result = get_user(State(state), Path(non_existent_id)).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.0, StatusCode::NOT_FOUND);
    }
}
