use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;

/// Establish connection to database using Diesel
pub fn establish_connection() -> PgConnection {
    let database_url = env::var("POSTGRES_CONN_STRING")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}