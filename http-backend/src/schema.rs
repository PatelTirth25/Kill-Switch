// @generated automatically by Diesel CLI.

diesel::table! {
    rooms (id) {
        id -> Int4,
        room -> Varchar,
        url -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    rooms,
    users,
);
