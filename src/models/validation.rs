use regex::Regex;
use crate::errors::AuthError;

/// Checks if the email has the correct format
///
/// Valid examples:
/// - user@email.com
/// - name.surname@company.com.br
/// - test123@provider.co
///
/// Invalid examples:
/// - email@
/// - @email.com
/// - email.com
pub fn validate_email(email: &str) -> Result<(), AuthError> {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

    if !email_regex.is_match(email){
        return Err(AuthError::ValidationError("Invalid email format!".to_string()));
    }

    if email.len() > 255 {
        return Err(AuthError::ValidationError("Email is to long (max 255 characters)".to_string()));
    }
    Ok(())
}



/// Validates if the username is valid
///
/// Rules:
/// - Minimum 3 characters
/// - Maximum 50 characters
/// - Only letters, numbers, underscores, and hyphens
/// - Cannot start or end with an underscore/hyphen
pub fn validate_username(username: &str) -> Result<(), AuthError> {
    if username.len() < 3 {
        return Err(AuthError::ValidationError("Username must be at least 3 characters long".to_string()));
    }

    if username.len() > 50 {
        return Err(AuthError::ValidationError("Username is to long (max 50 characters)".to_string()));
    }

    let username_regex = Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9_-]*[a-zA-Z0-9]$").unwrap();
    if !username_regex.is_match(username) {
        return Err(AuthError::ValidationError(
            "Username can only contain letters, numbers, underscore and hyphen. Cannot start or end with special characters".to_string()
        ));
    }
    Ok(())
}



/// Validates if the password is strong enough
///
/// Strong password rules:
/// - Minimum 8 characters
/// - At least 1 uppercase letter (A-Z)
/// - At least 1 lowercase letter (a-z)
/// - At least 1 number (0-9)
/// - At least 1 special character (!@#$%^&*()_+-=[]{}|;:,.<>?)
pub fn validate_password(password: &str) -> Result<(), AuthError> {
    let mut errors = Vec::new();

    if password.len() < 8 {
        errors.push("at least 8 characters");
    }

    if !password.chars().any(|c| c.is_uppercase()) {
        errors.push("at least one uppercase letter (A-Z)");
    }

    if !password.chars().any(|c: char| c.is_lowercase()) {
        errors.push("at least one lowercase letter (a-z)");
    }

    if !password.chars().any(|c: char| c.is_numeric()){
        errors.push("at least one number (0-9)");
    }

    let special_chars = "!@#$%^&*()_+-=[]{}|;:,.<>?";
    if !password.chars().any(|c| special_chars.contains(c)) {
        errors.push("at least one special character");
    }

    if !errors.is_empty() {
        let error_msg = format!(
            "Password must contain: {}",
            errors.join(", ")
        );
        return Err(AuthError::ValidationError(error_msg))
    }

    Ok(())
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        assert!(validate_email("user@example.com").is_ok());
        assert!(validate_email("test.user@company.co.uk").is_ok());
        assert!(validate_email("name123@provider.org").is_ok());
    }

    #[test]
    fn test_invalid_email() {
        assert!(validate_email("invalid").is_err());
        assert!(validate_email("@example.com").is_err());
        assert!(validate_email("user@").is_err());
        assert!(validate_email("user@com").is_err());
    }

    #[test]
    fn test_valid_username() {
        assert!(validate_username("john_doe").is_ok());
        assert!(validate_username("user123").is_ok());
        assert!(validate_username("test-user").is_ok());
    }

    #[test]
    fn test_invalid_username() {
        assert!(validate_username("ab").is_err()); // muito curto
        assert!(validate_username("_user").is_err()); // começa com _
        assert!(validate_username("user_").is_err()); // termina com _
        assert!(validate_username("user name").is_err()); // tem espaço
    }

    #[test]
    fn test_valid_password() {
        assert!(validate_password("Senha123!").is_ok());
        assert!(validate_password("MyP@ssw0rd").is_ok());
        assert!(validate_password("Str0ng#Pass").is_ok());
    }

    #[test]
    fn test_invalid_password() {
        assert!(validate_password("weak").is_err()); // muito curta
        assert!(validate_password("noupppercase1!").is_err()); // sem maiúscula
        assert!(validate_password("NOLOWERCASE1!").is_err()); // sem minúscula
        assert!(validate_password("NoNumbers!").is_err()); // sem número
        assert!(validate_password("NoSpecial123").is_err()); // sem especial
    }
}