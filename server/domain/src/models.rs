use diesel::prelude::*;
use chrono::NaiveDateTime;
use infrastructure::schema::{language, orders, product, users, product_description, category, account, registration_confirmation, transaction, user_wallet, rating, liked, delivery_address, delivery_service};
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

#[derive(Queryable, Serialize, Insertable, AsChangeset, Debug, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub language: Option<i32>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<NaiveDateTime>,
    pub role: Option<String>,
    pub image: Option<String>,
    pub phone_number: Option<String>,
    pub user_info: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub last_order: Option<NaiveDateTime>,
    pub is_delete: bool,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
pub struct NewUser {
    pub language: Option<i32>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<NaiveDateTime>,
    pub role: Option<String>,
    pub image: Option<String>,
    pub phone_number: Option<String>,
    pub user_info: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
    pub last_order: Option<NaiveDateTime>,
    pub is_delete: bool,
}

#[derive(Queryable, Serialize, Insertable, AsChangeset, Debug, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(table_name = product)]
pub struct Product {
    pub id: i32,
    pub language: Option<i32>,
    pub category: Vec<Option<i32>>,
    pub description: Option<i32>,
    pub name: String,
    pub image: Option<String>,
    pub weight: Vec<Option<i32>>,
    pub price: i32,
    pub pieces: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_delete: bool,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = product)]
pub struct NewProduct {
    pub language: Option<i32>,
    pub category: Vec<Option<i32>>,
    pub description: Option<i32>,
    pub name: String,
    pub image: Option<String>,
    pub weight: Vec<Option<i32>>,
    pub price: i32,
    pub pieces: i32,
    pub updated_at: Option<NaiveDateTime>,
    pub is_delete: bool,
}

#[derive(Queryable, Serialize, Insertable, AsChangeset, Debug, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(table_name = orders)]
pub struct Order {
    pub id: i32,
    pub user_id: Option<i32>,
    pub delivery_address_id: Option<i32>,
    pub delivery_service_id: Option<i32>,
    pub transaction: Option<i32>,
    pub product_ids: Vec<Option<i32>>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub status: String,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = orders)]
pub struct NewOrder {
    pub user_id: Option<i32>,
    pub delivery_address_id: Option<i32>,
    pub delivery_service_id: Option<i32>,
    pub transaction: Option<i32>,
    pub product_ids: Vec<Option<i32>>,
    pub updated_at: Option<NaiveDateTime>,
    pub status: String,
}

#[derive(Queryable, Serialize, Insertable, AsChangeset, Debug, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(table_name = category)]
pub struct Category {
    pub id: i32,
    pub language: Option<i32>,
    pub name: String,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_delete: Option<bool>,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = category)]
pub struct NewCategory {
    pub language: i32,
    pub name: String,
    pub image: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_delete: bool,
}

#[derive(Queryable, Serialize, Insertable, AsChangeset, Debug, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(table_name = account)]
pub struct Account {
    pub id: i32,
    pub user_id: i32,
    pub account_type: Option<String>,
    pub provider: Option<String>,
    pub provider_user_id: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub expired_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = account)]
pub struct NewAccount {
    pub user_id: i32,
    pub account_type: String,
    pub provider: String,
    pub provider_user_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub expired_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Insertable, AsChangeset, Debug, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(table_name = registration_confirmation)]
pub struct RegistrationConfirmation {
    pub id: i32,
    pub user_id: i32,
    pub activation_token: String,
    pub expired_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = registration_confirmation)]
pub struct NewRegistrationConfirmation {
    pub user_id: i32,
    pub activation_token: String,
    pub expired_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Insertable, AsChangeset, Debug, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(table_name = transaction)]
pub struct Transaction {
    pub id: i32,
    pub user_id: i32,
    pub amount: i32,
    pub currency: String,
    pub transaction_type: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = transaction)]
pub struct NewTransaction {
    pub user_id: i32,
    pub amount: i32,
    pub currency: String,
    pub transaction_type: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Insertable, AsChangeset, Debug, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(table_name = language)]
pub struct Language {
    pub id: i32,
    pub short_name: Option<String>,
    pub full_name: Option<String>,
    pub image: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = language)]
pub struct NewLanguage {
    pub short_name: String,
    pub full_name: String,
    pub image: String,
}

#[derive(Queryable, Serialize, Insertable, AsChangeset, Debug, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(table_name = product_description)]
pub struct ProductDescription {
    pub id: i32,
    pub language: Option<i32>,
    pub name: String,
    pub image: Option<String>,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_delete: Option<bool>,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = product_description)]
pub struct NewProductDescription {
    pub language: i32,
    pub name: String,
    pub image: String,
    pub description: String,
    pub is_delete: bool,
}

#[derive(Queryable, Serialize, Insertable, AsChangeset, Debug, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(table_name = user_wallet)]
pub struct UserWallet {
    pub id: i32,
    pub user_id: i32,
    pub last_transaction: i32,
    pub amount: i32,
    pub currency: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = user_wallet)]
pub struct NewUserWallet {
    pub user_id: i32,
    pub last_transaction: i32,
    pub amount: i32,
    pub currency: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Insertable, AsChangeset, Debug, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(table_name = rating)]
pub struct Rating {
    pub id: i32,
    pub user_id: i32,
    pub product_id: i32,
    pub user_rating: i32,
    pub comment: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = rating)]
pub struct NewRating {
    pub user_id: i32,
    pub product_id: i32,
    pub user_rating: i32,
    pub comment: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Insertable, AsChangeset, Debug, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(table_name = liked)]
pub struct Liked {
    pub id: i32,
    pub user_id: i32,
    pub product_id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = liked)]
pub struct NewLiked {
    pub user_id: i32,
    pub product_id: i32,
    pub name: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Insertable, AsChangeset, Debug, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(table_name = delivery_address)]
pub struct DeliveryAddress {
    pub id: i32,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub country: String,
    pub city: String,
    pub region: String,
    pub street: String,
    pub building_number: String,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub comment: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = delivery_address)]
pub struct NewDeliveryAddress {
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub country: String,
    pub city: String,
    pub region: String,
    pub street: String,
    pub building_number: String,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub comment: Option<String>,
    pub created_at: NaiveDateTime,
}


#[derive(Queryable, Serialize, Insertable, AsChangeset, Debug, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(table_name = delivery_service)]
pub struct DeliveryService {
    pub id: i32,
    pub delivery_service_details: String,
    pub name: Option<String>,
    pub country: String,
    pub phone_number: Option<String>,
    pub comment: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = delivery_service)]
pub struct NewDeliveryService {
    pub delivery_service_details: String,
    pub name: Option<String>,
    pub country: String,
    pub phone_number: Option<String>,
    pub comment: Option<String>,
    pub created_at: NaiveDateTime,
}
