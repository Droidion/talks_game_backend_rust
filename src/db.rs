use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
use crate::models::User;

/// Establish connection to database using Diesel
fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

/// Get User by their login from the database
pub fn get_user_by_login(user_login: &str) -> Result<User, &str> {
    use crate::schema::users::dsl::*;
    let connection = establish_connection();
    let results = users
        .filter(login.eq(user_login))
        .load::<User>(&connection);
    match results {
        Ok(res) if res.is_empty() => Err("User not found"),
        Ok(res) if res.len() > 1 => Err("More than one user in database"),
        Ok(res) => Ok(res.first().unwrap().clone()),
        Err(_) => Err("DB error while searching for the user"),
    }
}
