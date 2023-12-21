use domain::models::{Order, Product, User};
use rocket::serde::Serialize;

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    User(User),
    Users(Vec<User>),
    Product(Product),
    Products(Vec<Product>),
    Order(Order),
    Orders(Vec<Order>),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}
