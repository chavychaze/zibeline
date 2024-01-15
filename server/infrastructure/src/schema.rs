// @generated automatically by Diesel CLI.

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
    users,
);
