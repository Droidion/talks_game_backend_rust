use chrono::Utc;

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

#[derive(Queryable, Clone, Debug)]
pub struct Session {
    pub id: i64,
    pub token: String,
    pub team_number: i32,
    pub team_type: String,
    pub is_commander: bool,
    pub inserted_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

