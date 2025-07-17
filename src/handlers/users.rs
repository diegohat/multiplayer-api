use crate::db::models::user::User;
use crate::error::status_code_from_sqlx_error;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use sqlx::{Executor, PgPool, Row, query};

#[axum::debug_handler]
pub async fn create_user(
    State(pool): State<PgPool>,
    Json(body): Json<User>,
) -> Result<Json<User>, (StatusCode, String)> {
    let result = pool.fetch_one(
        query("INSERT INTO users (email, username, password) VALUES ($1, $2, $3) RETURNING email, username, password")
            .bind(&body.email)
            .bind(&body.username)
            .bind(&body.password),
    )
    .await
    .map_err(|e| (status_code_from_sqlx_error(&e), e.to_string()))?;

    Ok(Json(User {
        email: result
            .try_get("email")
            .map_err(|e| (status_code_from_sqlx_error(&e), e.to_string()))?,
        username: result
            .try_get("username")
            .map_err(|e| (status_code_from_sqlx_error(&e), e.to_string()))?,
        password: result
            .try_get("password")
            .map_err(|e| (status_code_from_sqlx_error(&e), e.to_string()))?,
    }))
}

#[axum::debug_handler]
pub async fn read_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<User>, (StatusCode, String)> {
    let result = pool
        .fetch_one(query("SELECT email, username, password FROM users WHERE id = $1").bind(id))
        .await
        .map_err(|e| (status_code_from_sqlx_error(&e), e.to_string()))?;
    Ok(Json(User {
        email: result
            .try_get("email")
            .map_err(|e| (status_code_from_sqlx_error(&e), e.to_string()))?,
        username: result
            .try_get("username")
            .map_err(|e| (status_code_from_sqlx_error(&e), e.to_string()))?,
        password: result
            .try_get("password")
            .map_err(|e| (status_code_from_sqlx_error(&e), e.to_string()))?,
    }))
}

#[axum::debug_handler]
pub async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(body): Json<User>,
) -> Result<Json<User>, (StatusCode, String)> {
    let result = pool.fetch_one(
        query("UPDATE users SET email = $1, username = $2, password = $3, updated_at = now() WHERE id = $4 RETURNING email, username, password")
            .bind(&body.email)
            .bind(&body.username)
            .bind(&body.password)
            .bind(id),
    )
    .await
    .map_err(|e| (status_code_from_sqlx_error(&e), e.to_string()))?;

    Ok(Json(User {
        email: result
            .try_get("email")
            .map_err(|e| (status_code_from_sqlx_error(&e), e.to_string()))?,
        username: result
            .try_get("username")
            .map_err(|e| (status_code_from_sqlx_error(&e), e.to_string()))?,
        password: result
            .try_get("password")
            .map_err(|e| (status_code_from_sqlx_error(&e), e.to_string()))?,
    }))
}

#[axum::debug_handler]
pub async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<DateTime<Utc>>, (StatusCode, String)> {
    let result =
        sqlx::query("UPDATE users SET deleted_at = now() WHERE id = $1 RETURNING deleted_at")
            .bind(id)
            .fetch_one(&pool)
            .await
            .map_err(|e| (status_code_from_sqlx_error(&e), e.to_string()))?;
    Ok(Json(result.try_get("deleted_at").map_err(|e| {
        (status_code_from_sqlx_error(&e), e.to_string())
    })?))
}
