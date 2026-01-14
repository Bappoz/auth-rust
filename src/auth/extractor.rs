use crate::auth::jwt::Claims; // Importe o segredo aqui 
use crate::AppState;
use axum::{ 
    extract::{FromRequestParts, FromRef}, 
    http::request::Parts,
    http::StatusCode, 
};

// Struct that represents a autheticated user
pub struct AuthUser {
    pub user_id: String,
}

// Allow use AuthUser as a parameter in Axum handlers
impl<S> FromRequestParts<S> for AuthUser where AppState: FromRef<S>, S: Send + Sync {
    type Rejection = (StatusCode, String);  // Defining Fallback

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        // Search for the header
        let auth_header = parts
            .headers
            .get("Authorization") 
            .and_then(|h| h.to_str().ok())              // Try to convert into string
            .ok_or((StatusCode::UNAUTHORIZED, "Missing Token".to_string()))?; // Activates fallbakc
        
        // Check if start with "Bearer "
        if !auth_header.starts_with("Bearer ") {
            return Err((StatusCode::UNAUTHORIZED, "Invalid Token Format".into()));
        }

        // Removes "Bearer " and stores the token
        let token = &auth_header[7..];

        //Validar o token using AppState secret
        let token_data = jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(app_state.jwt_secret.as_bytes()),
            &jsonwebtoken::Validation::default(),
        ).map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid Token or expired".into()))?;

        // Return the user authenticated
        Ok(AuthUser { user_id: token_data.claims.sub })
    }
}