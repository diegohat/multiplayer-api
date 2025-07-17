use crate::models::users::User;
use async_trait::async_trait;
use axum::{Json, http::StatusCode};
use chrono::{DateTime, Utc};

#[async_trait]
pub trait UsersServiceTrait {
    async fn create_user(&self, user: User) -> Result<Json<User>, (StatusCode, String)>;
    async fn read_user(&self, user_id: i32) -> Result<Json<User>, (StatusCode, String)>;
    async fn update_user(&self, user_id: i32, user: User) -> Result<Json<User>, (StatusCode, String)>;
    async fn delete_user(&self, user_id: i32) -> Result<Json<DateTime<Utc>>, (StatusCode, String)>;
}
