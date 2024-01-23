use chrono::NaiveDateTime;
use domain::models::{Product, Order, User};
use serde::Serialize;

#[derive(Serialize, Debug, Ord, Eq, PartialEq, PartialOrd)]
pub struct AddressedMail {
    pub id: i32,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub sender: Option<String>,
    pub receiver: Option<String>,
    pub sent_at: NaiveDateTime,
    pub is_delete: Option<bool>,
    pub is_read: Option<bool>,
}

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    Product(Product),
    Products(Vec<Product>),
    Order(Order),
    Orders(Vec<Order>),
    User(User),
    Users(Vec<User>),
}

#[derive(Serialize)]
pub struct Response {
    pub body: ResponseBody,
}
