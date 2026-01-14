use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use jsonwebtoken::{
    encode,
    decode,
    Header,
    Validation,
    EncodingKey,
    DecodingKey
};

// Data stored in JWT token
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,    // User Id
    pub exp: usize,       // Expiration time
    pub iat: usize,       // Issued at
}

/// Creates a new JWT token for user
pub fn create_token(user_id: &str, secret: &str) -> String {
    let now = Utc::now();
    // Validates token for 24 hours  
    let expire = now + Duration::hours(24);

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expire.timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    // Encode and sign the token
    encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(secret.as_ref()),
    ).expect("Error generating token")
}

/// Validate and decode the JWT token
/// Args:
///     token - Token JWT beeing validated
///     secret - Secret used for verifying
/// 
/// Returns: Claims if the Token is valid, Error otherwise
pub fn validate_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default()
    )?;

    Ok(token_data.claims)
}