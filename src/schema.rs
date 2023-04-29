// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        price -> Numeric,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        name -> Varchar,
        password -> Varchar,
        createdat -> Timestamp,
        updatedat -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    products,
    users,
);
