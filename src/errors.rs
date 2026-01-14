use axum::{
    http::StatusCode, 
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("UserAlreadyExists")]
    UserAlreadyExists,

    #[error("User not found")]
    UserNotFound,

    #[error("Invalid Token")]
    InvalidToken,

    #[error("Token expired")]
    TokenExpired,

    #[error("Database error")]
    DatabaseError,
    
    #[error("Internal server error")]
    InternalError,

    #[error("Validation error: {0}")]
    ValidationError(String)
}


impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()),
            AuthError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists".to_string()),
            AuthError::UserNotFound => (StatusCode::NOT_FOUND, "User not found".to_string()),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token".to_string()),
            AuthError::TokenExpired => (StatusCode::UNAUTHORIZED, "Token expired".to_string()),
            AuthError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()),
            AuthError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
            AuthError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let body = Json(json!({
            "error": message
        }));

        (status, body).into_response()
    }
}
