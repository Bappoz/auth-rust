use axum::{Json, extract::State};
use crate::{
    models::auth::{LoginRequest, LoginResponse, RegisterRequest},
    models::user::CreateUser,
    models::validation::{validate_email, validate_username, validate_password},
    auth::{crypto, jwt::create_token},
    errors::AuthError,
    AppState,
};

/// Handler for registering new users
/// 
/// Endpoint: POST /register
/// Body: {"username": "...", "email": "...", "password": "..."}
/// 
/// Flow:
/// 1. Checks if email already exists
/// 2. Checks if username already exists
/// 3. Hash the password with Argon2
/// 4. Creates the user in the database
/// 5. Generates JWT token
/// 6. Returns the token
/// 
/// This handler is GENERIC - it doesn't know which bank is being used!
pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<LoginResponse>, AuthError> {

    // Validation
    validate_email(&payload.email)?;
    validate_username(&payload.username)?;
    validate_password(&payload.password)?;

    // Check if the email is already in use
    if state.user_repo.find_by_email(&payload.email).await?.is_some() {
        return Err(AuthError::UserAlreadyExists);
    }

    // Check if the username is already in use
    if state.user_repo.find_by_username(&payload.username).await?.is_some() {
        return Err(AuthError::UserAlreadyExists);
    }

    // Generates a safe hash for the password using Argon2
    let password_hash = crypto::hash_password(&payload.password)
        .map_err(|_| AuthError::InternalError)?;

    // Creater user in db via trait UserRepository
    let user = state.user_repo.create(
        CreateUser{
            username: payload.username.clone(),
            email: payload.email,
            password: payload.password,
        }, 
        password_hash,
    ).await?;

    // Generate valid jwt token for 24 hours
    let token = create_token(&user.id.to_string(), &state.jwt_secret);

    // Return a token for the client
    Ok(Json(LoginResponse { token }))    
}


/// Handler for logging in existing users
/// 
/// Endpoint: POST /login
/// Body: {"username": "...", "password": "..."}
/// 
/// Flow:
/// 1. User search for username
/// 2. Checks if the password is correct
/// 3. Generates JWT token
/// 4. Returns the token
/// 
/// This handler is GENERIC - it doesn't know which bank is being used!
pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AuthError> {

    let user = state.user_repo
        .find_by_username(&payload.username)
        .await?
        .ok_or(AuthError::InvalidCredentials)?;

    let is_valid = crypto::verify_password(&user.password_hash, &payload.password)
        .map_err(|_| AuthError::InternalError)?;
    
    if !is_valid {
        return Err(AuthError::InvalidCredentials);
    }
    
    // Generate valid JWT token for 24 hours
    let token = create_token(&user.id.to_string(), &state.jwt_secret);

    Ok(Json(LoginResponse { token }))
}



