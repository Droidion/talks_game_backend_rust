extern crate argonautica;

use argonautica::{Hasher, Verifier};
use uuid::Uuid;
use crate::models::Session;
use crate::db::get_user_by_login;
use crate::cache::{add_session, clear_session};
use chrono::prelude::*;

/// Generates new UUID
pub fn generate_uuid() -> String {
    return Uuid::new_v4()
        .to_hyphenated()
        .to_string();
}

/// Password hash
pub fn generate_hash(password: &str) -> String {
    let mut hasher = Hasher::default();
    let hash = hasher
        .opt_out_of_secret_key(true)
        .with_password(password)
        .hash()
        .unwrap();
    hash
}

/// Check if given user's password matches the hash
fn password_matches(hash: &str, pass: &str) -> Result<bool, &'static str> {
    let mut verifier = Verifier::default();
    let is_valid = verifier
        .with_hash(hash)
        .with_password(pass)
        .verify();
    match is_valid {
        Ok(true) => Ok(true),
        Ok(_) => Err("Password does not match"),
        Err(_) => Err("Could validate the password"),
    }
}

/// Sign in a user
pub fn sign_in(login: &str, password: &str) -> Result<Session, &'static str> {
    let user = get_user_by_login(login)?;
    password_matches(&user.password, password)?;
    let uuid = generate_uuid();
    let session = Session {
        token: uuid.clone(),
        team_number: user.team_number,
        team_type: user.team_type,
        is_commander: false,
        inserted_at: Utc::now(),
        updated_at: Utc::now()
    };
    add_session(&uuid, &session)?;
    Ok(session)
}

/// Sign out a user
pub fn sign_out(token: &str) -> Result<(), &'static str> {
    // TODO add clear cache from Redis
    clear_session(token)?;
    Ok(())
}