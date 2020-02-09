extern crate redis;
use crate::models::Session;
use redis::Commands;
use redis::Connection;
use std::env;
use self::redis::RedisResult;

fn establish_connection() -> Connection {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let client = redis::Client::open(redis_url).expect("Could not create Redis client");
    client
        .get_connection()
        .expect("Could not establish connection to Redis")
}

/// Add a session to redis cache
pub fn add_session(token: &str, session: &Session) -> Result<(), &'static str> {
    let mut con = establish_connection();
    let session_serialized = serde_json::to_string::<Session>(session)
        .expect("Could not serialize session");
    let res: RedisResult<()> = con.set(["session:", token].concat(), session_serialized);
    match res {
        Err(_) => Err("Could not cache session to Redis"),
        Ok(_) => Ok(()),
    }
}

/// Clear a session from redis cache
pub fn clear_session(token: &str) -> Result<(), &'static str> {
    let mut con = establish_connection();
    let res: RedisResult<i32> = con.del(["session:", token].concat());
    match res {
        Err(_) => Err("Could not clear session with given token from Redis"),
        Ok(0) => Err("Token was not found in cache and was not deleted"),
        Ok(_) => Ok(()),
    }
}
