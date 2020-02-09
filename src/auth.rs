extern crate argonautica;

use argonautica::{Hasher, Verifier};
use uuid::Uuid;

/// Generates new UUID
pub fn generate_uuid() -> Uuid {
    return Uuid::new_v4();
}

/// Password hash
pub fn generate_hash(password: &str) -> String {
    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(password)
        .hash()
        .unwrap();
    hash
}

/// Check if given user's password matches the hash
pub fn password_matches<'a>(hash: &str, pass: &str) -> Result<(), &'a str> {
    let mut verifier = Verifier::default();
    let is_valid = verifier
        .with_hash(hash)
        .with_password(pass)
        .verify();
    match is_valid {
        Ok(true) => Ok(()),
        Ok(_) => Err("Password does not match"),
        Err(_) => Err("Could validate the password"),
    }
}
