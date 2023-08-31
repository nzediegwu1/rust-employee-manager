// @generated automatically by Diesel CLI.

diesel::table! {
    employees (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Nullable<Varchar>,
        department -> Varchar,
        salary -> Int4,
        age -> Int2,
        created_at -> Timestamp,
    }
}
