table! {
    users (id) {
        id -> Int8,
        team_number -> Int4,
        team_type -> Varchar,
        login -> Varchar,
        password -> Varchar,
        inserted_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
