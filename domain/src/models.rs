use infrastructure::schema::{ users, products, orders };
use diesel::prelude::*;
use std::cmp::{ Ord, Eq, PartialOrd, PartialEq };
use rocket::serde::{ Deserialize, Serialize };
use chrono::NaiveDateTime;

// Representation of the 'users' table
#[derive(Queryable, Serialize, Insertable, AsChangeset, Debug, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub last_order: NaiveDateTime,
    pub is_registered: bool,
    pub is_delete: bool,
}

// Insertable structure for 'users' table
#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub address: String,
    pub phone: String,
    pub is_registered: bool,
}

// Representation of the 'products' table
#[derive(Queryable, Serialize, Insertable, AsChangeset, Debug, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(table_name = products)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: i32,
    pub pieces: i32,
    pub pieces_left: i32,
    pub group_ids: Vec<Option<i32>>,
    pub last_order: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_delete: Option<bool>,
}

// Insertable structure for 'products' table
#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub name: String,
    pub description: String,
    pub price: i32,
    pub pieces_left: i32,
    pub group_ids: Vec<i32>,
}

// Representation of the 'orders' table
#[derive(Queryable, Serialize, Insertable, AsChangeset, Debug, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(table_name = orders)]
pub struct Order {
    pub id: i32,
    pub user_id: Option<i32>,
    pub product_ids: Vec<Option<i32>>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_formed: Option<bool>,
    pub is_sent: Option<bool>,
    pub is_delivered: bool,
    pub is_delete: bool,
}

// Insertable structure for 'orders' table
#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = orders)]
pub struct NewOrder {
    pub user_id: i32,
    pub product_ids: Vec<i32>,
}
