//! # Main library

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

pub mod auth;
pub mod cache;
pub mod db;
pub mod models;
pub mod schema;
pub mod web;

use web::start;

pub fn start_server() {
    start();
}
