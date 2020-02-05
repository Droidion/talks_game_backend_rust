extern crate argonautica;

use argonautica::Hasher;
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
        .with_secret_key("\
            secret key that you should really store in a .env file \
            instead of in code, but this is just an example\
        ")
        .hash()
        .unwrap();
    hash
}