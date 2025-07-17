use axum::http::StatusCode;
use sqlx::Error;

pub fn status_code_from_sqlx_error(err: &Error) -> StatusCode {
    match err {
        Error::RowNotFound => StatusCode::NOT_FOUND,
        Error::Database(_) => StatusCode::BAD_REQUEST,
        Error::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
        Error::Tls(_) => StatusCode::INTERNAL_SERVER_ERROR,
        Error::Protocol(_) => StatusCode::INTERNAL_SERVER_ERROR,
        Error::PoolTimedOut => StatusCode::GATEWAY_TIMEOUT,
        Error::PoolClosed => StatusCode::SERVICE_UNAVAILABLE,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
