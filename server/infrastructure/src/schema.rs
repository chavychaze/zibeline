// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;
}

diesel::table! {
    orders (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        product_ids -> Array<Nullable<Int4>>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_formed -> Nullable<Bool>,
        is_sent -> Nullable<Bool>,
        is_delivered -> Bool,
        is_delete -> Bool,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        price -> Int4,
        pieces -> Int4,
        pieces_left -> Int4,
        group_ids -> Array<Nullable<Int4>>,
        last_order -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_delete -> Nullable<Bool>,
    }
}

diesel::table! {
    user_roles (id) {
        id -> Int4,
        user_role -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserRole;

    users (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        #[max_length = 100]
        email -> Nullable<Varchar>,
        #[max_length = 255]
        address -> Nullable<Varchar>,
        #[max_length = 15]
        phone -> Nullable<Varchar>,
        role -> UserRole,
        #[max_length = 255]
        password_hash -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        last_order -> Timestamp,
        is_registered -> Bool,
        is_delete -> Bool,
    }
}

diesel::joinable!(orders -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    orders,
    products,
    user_roles,
    users,
);
