// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 255]
        account_type -> Nullable<Varchar>,
        #[max_length = 100]
        provider -> Nullable<Varchar>,
        #[max_length = 255]
        provider_user_id -> Nullable<Varchar>,
        #[max_length = 255]
        access_token -> Nullable<Varchar>,
        #[max_length = 255]
        refresh_token -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        expired_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    category (id) {
        id -> Int4,
        language -> Nullable<Int4>,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        image -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_delete -> Nullable<Bool>,
    }
}

diesel::table! {
    delivery_address (id) {
        id -> Int4,
        #[max_length = 100]
        first_name -> Nullable<Varchar>,
        #[max_length = 100]
        middle_name -> Nullable<Varchar>,
        #[max_length = 100]
        last_name -> Nullable<Varchar>,
        #[max_length = 100]
        country -> Varchar,
        #[max_length = 100]
        city -> Varchar,
        #[max_length = 100]
        region -> Varchar,
        #[max_length = 100]
        street -> Varchar,
        #[max_length = 100]
        building_number -> Varchar,
        #[max_length = 255]
        address -> Nullable<Varchar>,
        #[max_length = 100]
        phone_number -> Nullable<Varchar>,
        #[max_length = 255]
        comment -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    delivery_service (id) {
        id -> Int4,
        #[max_length = 255]
        delivery_service_details -> Varchar,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        #[max_length = 100]
        country -> Varchar,
        #[max_length = 100]
        phone_number -> Nullable<Varchar>,
        #[max_length = 255]
        comment -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    language (id) {
        id -> Int4,
        #[max_length = 15]
        short_name -> Nullable<Varchar>,
        #[max_length = 100]
        full_name -> Nullable<Varchar>,
        #[max_length = 255]
        image -> Nullable<Varchar>,
    }
}

diesel::table! {
    liked (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        product_id -> Nullable<Int4>,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        delivery_address_id -> Nullable<Int4>,
        delivery_service_id -> Nullable<Int4>,
        transaction -> Nullable<Int4>,
        product_ids -> Array<Nullable<Int4>>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 50]
        status -> Varchar,
    }
}

diesel::table! {
    product (id) {
        id -> Int4,
        language -> Nullable<Int4>,
        category -> Array<Nullable<Int4>>,
        description -> Nullable<Int4>,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        image -> Nullable<Varchar>,
        weight -> Array<Nullable<Int4>>,
        price -> Int4,
        pieces -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_delete -> Bool,
    }
}

diesel::table! {
    product_description (id) {
        id -> Int4,
        language -> Nullable<Int4>,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        image -> Nullable<Varchar>,
        #[max_length = 255]
        description -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_delete -> Nullable<Bool>,
    }
}

diesel::table! {
    rating (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        product_id -> Nullable<Int4>,
        user_rating -> Int4,
        #[max_length = 255]
        comment -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    registration_confirmation (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 255]
        activation_token -> Varchar,
        expired_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    transaction (id) {
        id -> Int4,
        user_id -> Int4,
        amount -> Int4,
        #[max_length = 15]
        currency -> Varchar,
        #[max_length = 50]
        transaction_type -> Varchar,
        #[max_length = 50]
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user_wallet (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        last_transaction -> Nullable<Int4>,
        #[max_length = 50]
        currency -> Varchar,
        amount -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        language -> Nullable<Int4>,
        #[max_length = 100]
        first_name -> Nullable<Varchar>,
        #[max_length = 100]
        last_name -> Nullable<Varchar>,
        #[max_length = 100]
        email -> Nullable<Varchar>,
        email_verified -> Nullable<Timestamp>,
        #[max_length = 15]
        role -> Nullable<Varchar>,
        #[max_length = 100]
        image -> Nullable<Varchar>,
        #[max_length = 15]
        phone_number -> Nullable<Varchar>,
        #[max_length = 255]
        user_info -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        last_order -> Nullable<Timestamp>,
        is_delete -> Bool,
    }
}

diesel::joinable!(account -> users (user_id));
diesel::joinable!(category -> language (language));
diesel::joinable!(liked -> product (product_id));
diesel::joinable!(liked -> users (user_id));
diesel::joinable!(orders -> delivery_address (delivery_address_id));
diesel::joinable!(orders -> delivery_service (delivery_service_id));
diesel::joinable!(orders -> transaction (transaction));
diesel::joinable!(orders -> users (user_id));
diesel::joinable!(product -> language (language));
diesel::joinable!(product -> product_description (description));
diesel::joinable!(product_description -> language (language));
diesel::joinable!(rating -> product (product_id));
diesel::joinable!(rating -> users (user_id));
diesel::joinable!(registration_confirmation -> users (user_id));
diesel::joinable!(transaction -> users (user_id));
diesel::joinable!(user_wallet -> transaction (last_transaction));
diesel::joinable!(user_wallet -> users (user_id));
diesel::joinable!(users -> language (language));

diesel::allow_tables_to_appear_in_same_query!(
    account,
    category,
    delivery_address,
    delivery_service,
    language,
    liked,
    orders,
    product,
    product_description,
    rating,
    registration_confirmation,
    transaction,
    user_wallet,
    users,
);
