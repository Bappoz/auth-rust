// This file is responsible for the password protection using Argon2id, 
    // for password hashing

use argon2::{
    Argon2, password_hash::{
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng
    }
};

// Generates a hash for a password using Argon2
// Args: 'password' - string
// Returns: String with password's hash, including salt and parameters
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    // Generate the hash
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;

    // Returns hash as a string
    Ok(password_hash.to_string())
}

pub fn verify_password(hash: &str, password: &str) -> Result<bool, argon2::password_hash::Error> {
    // Store parsed hash
    let parsed_hash = PasswordHash::new(hash)?;

    // Create Argo2 instance
    let argon2 = Argon2::default();

    // Verify if the password correpond to the hash
    Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
}