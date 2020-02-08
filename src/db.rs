use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
use crate::models::User;

/// Establish connection to database using Diesel
pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn get_users() -> () {
    use crate::schema::users::dsl::*;
    let connection = establish_connection();
    let results = users.load::<User>(&connection).expect("Error loading users");
    for user in results {
        println!("{:?}", user.login.unwrap());
    }
}