use crate::models::users::User;
use crate::services::UsersService;
use crate::services::traits::UsersServiceTrait;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};

#[axum::debug_handler]
pub async fn create_user(
    State(service): State<UsersService>,
    Json(body): Json<User>,
) -> Result<Json<User>, (StatusCode, String)> {
    service.create_user(body).await
}

#[axum::debug_handler]
pub async fn read_user(
    State(service): State<UsersService>,
    Path(id): Path<i32>,
) -> Result<Json<User>, (StatusCode, String)> {
    service.read_user(id).await
}

#[axum::debug_handler]
pub async fn update_user(
    State(service): State<UsersService>,
    Path(id): Path<i32>,
    Json(body): Json<User>,
) -> Result<Json<User>, (StatusCode, String)> {
    service.update_user(id, body).await
}

#[axum::debug_handler]
pub async fn delete_user(
    State(service): State<UsersService>,
    Path(id): Path<i32>,
) -> Result<Json<DateTime<Utc>>, (StatusCode, String)> {
    service.delete_user(id).await
}
