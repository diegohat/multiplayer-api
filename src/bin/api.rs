use axum::{
    Router,
    routing::{delete, get, patch, post},
    serve,
};
use dotenvy::dotenv;
use api::{
    handlers::users::{create_user, delete_user, read_user, update_user},
    repositories::UsersRepository,
    services::UsersService,
};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::{env, time::Duration};
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let options = PgConnectOptions::new()
        .host(&env::var("PG_HOST").unwrap_or_else(|_| "localhost".to_string()))
        .port(
            env::var("PG_PORT")
                .unwrap_or_else(|_| "5432".to_string())
                .parse()
                .unwrap_or(5432),
        )
        .username(&env::var("PG_USER").unwrap_or_else(|_| "postgres".to_string()))
        .password(&env::var("PG_PASSWORD").unwrap_or_else(|_| "postgres".to_string()))
        .database(&env::var("PG_DB").unwrap_or_else(|_| "mp-api".to_string()));

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(10))
        .connect_with(options)
        .await
        .expect("Failed to connect to the database");

    let users_repo = UsersRepository::new(pool.clone());
    let users_service = UsersService::new(users_repo);

    let app = Router::new()
        .route("/users", post(create_user))
        .route("/users/{id}", get(read_user))
        .route("/users/{id}", patch(update_user))
        .route("/users/{id}", delete(delete_user))
        .with_state(users_service);

    let listener = TcpListener::bind("localhost:3000")
        .await
        .expect("Failed to bind to address");
    info!("Server running on http://localhost:3000");
    serve(listener, app).await.expect("Server error");
}
