use crate::models::users::User;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::Error;

#[async_trait]
pub trait UsersRepositoryTrait {
    async fn create_user(&self, user: User) -> Result<User, Error>;
    async fn read_user(&self, user_id: i32) -> Result<User, Error>;
    async fn update_user(&self, user_id: i32, user: User) -> Result<User, Error>;
    async fn delete_user(&self, user_id: i32) -> Result<DateTime<Utc>, Error>;
}
