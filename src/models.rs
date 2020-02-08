#[derive(Queryable)]
pub struct User {
    pub id: i64,
    pub team_number: Option<i32>,
    pub team_type: Option<String>,
    pub login: Option<String>,
    pub password: Option<String>,
    pub inserted_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
