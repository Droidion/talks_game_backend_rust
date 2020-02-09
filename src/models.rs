use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Clone, Debug)]
pub struct User {
    pub id: i64,
    pub team_number: i32,
    pub team_type: String,
    pub login: String,
    pub password: String,
    pub inserted_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Clone, Debug, Serialize, Deserialize, juniper::GraphQLObject)]
pub struct Session {
    pub token: String,
    pub team_number: i32,
    pub team_type: String,
    pub is_commander: bool,
    pub inserted_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

