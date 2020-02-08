table! {
    users (id) {
        id -> Int8,
        team_number -> Nullable<Int4>,
        team_type -> Nullable<Varchar>,
        login -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        inserted_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
