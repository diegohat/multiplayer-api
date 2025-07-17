use crate::{
    error::status_code_from_sqlx_error, models::users::User,
    repositories::traits::UsersRepositoryTrait, services::traits::UsersServiceTrait,
};
use async_trait::async_trait;
use axum::{Json, http::StatusCode};
use chrono::{DateTime, Utc};
use std::sync::Arc;

#[derive(Clone)]
pub struct UsersService {
    users_repo: Arc<dyn UsersRepositoryTrait + Send + Sync>,
}

impl UsersService {
    pub fn new(users_repo: Arc<dyn UsersRepositoryTrait + Send + Sync>) -> Self {
        Self { users_repo }
    }
}

#[async_trait]
impl UsersServiceTrait for UsersService {
    async fn create_user(&self, user: User) -> Result<Json<User>, (StatusCode, String)> {
        match self.users_repo.create_user(user).await {
            Ok(user) => Ok(Json(user)),
            Err(e) => {
                return Err((status_code_from_sqlx_error(&e), e.to_string()));
            }
        }
    }

    async fn read_user(&self, user_id: i32) -> Result<Json<User>, (StatusCode, String)> {
        match self.users_repo.read_user(user_id).await {
            Ok(user) => Ok(Json(user)),
            Err(e) => {
                return Err((status_code_from_sqlx_error(&e), e.to_string()));
            }
        }
    }

    async fn update_user(&self, user_id: i32, user: User) -> Result<Json<User>, (StatusCode, String)> {
        match self.users_repo.update_user(user_id, user).await {
            Ok(user) => Ok(Json(user)),
            Err(e) => {
                return Err((status_code_from_sqlx_error(&e), e.to_string()));
            }
        }
    }

    async fn delete_user(&self, user_id: i32) -> Result<Json<DateTime<Utc>>, (StatusCode, String)> {
        match self.users_repo.delete_user(user_id).await {
            Ok(deleted_at) => Ok(Json(deleted_at)),
            Err(e) => {
                return Err((status_code_from_sqlx_error(&e), e.to_string()));
            }
        }
    }
}
