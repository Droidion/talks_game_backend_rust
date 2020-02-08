//! # Main library
#[macro_use]
extern crate diesel;

pub mod auth;
pub mod db;
pub mod models;
pub mod schema;

pub fn start_server() {
    println!("Starting the server...")
}
