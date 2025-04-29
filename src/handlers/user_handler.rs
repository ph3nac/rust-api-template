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
