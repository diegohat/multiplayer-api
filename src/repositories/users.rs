use std::sync::Arc;

use crate::{
    models::users::User,
    repositories::{
        sql::{CREATE_USER_QUERY, DELETE_USER_QUERY, READ_USER_QUERY, UPDATE_USER_QUERY},
        traits::UsersRepositoryTrait,
    },
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Error, Executor, PgPool, Row, postgres::PgRow, query};

pub struct UsersRepository {
    pool: PgPool,
}

impl UsersRepository {
    pub fn new(pool: PgPool) -> Arc<Self> {
        Arc::new(Self { pool })
    }
    pub fn extract_user(&self, db_row: PgRow) -> Result<User, Error> {
        Ok(User {
            email: db_row.try_get("email")?,
            username: db_row.try_get("username")?,
            password: db_row.try_get("password")?,
        })
    }
}

#[async_trait]
impl UsersRepositoryTrait for UsersRepository {
    async fn create_user(&self, user: User) -> Result<User, Error> {
        let db_row = self
            .pool
            .fetch_one(
                query(CREATE_USER_QUERY)
                    .bind(&user.email)
                    .bind(&user.username)
                    .bind(&user.password),
            )
            .await?;
        let user = self.extract_user(db_row)?;
        Ok(user)
    }

    async fn read_user(&self, user_id: i32) -> Result<User, Error> {
        let db_row = self
            .pool
            .fetch_one(query(READ_USER_QUERY).bind(user_id))
            .await?;
        let user = self.extract_user(db_row)?;
        Ok(user)
    }

    async fn update_user(&self, user_id: i32, user: User) -> Result<User, Error> {
        let db_row = self
            .pool
            .fetch_one(
                query(UPDATE_USER_QUERY)
                    .bind(&user.email)
                    .bind(&user.username)
                    .bind(&user.password)
                    .bind(user_id),
            )
            .await?;
        let user = self.extract_user(db_row)?;
        Ok(user)
    }

    async fn delete_user(&self, user_id: i32) -> Result<DateTime<Utc>, Error> {
        let db_row = self
            .pool
            .fetch_one(query(DELETE_USER_QUERY).bind(user_id))
            .await?;
        Ok(db_row.try_get("deleted_at")?)
    }
}
