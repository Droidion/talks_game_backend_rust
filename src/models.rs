#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub team_number: String,
    pub body: String,
    pub published: bool,
}