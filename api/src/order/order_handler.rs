use shared::response_models::{Response, ResponseBody};
use application::order::{create, read};
use domain::models::{Order, NewOrder};
use rocket::{get, post};
use rocket::response::status::{NotFound, Created};
use rocket::serde::json::Json;

#[get("/orders")]
pub fn list_orders_handler() -> String {
    let orders: Vec<Order> = read::list_orders();
    let response = Response { body: ResponseBody::Orders(orders) };

    serde_json::to_string(&response).unwrap()
}

#[get("/order/<order_id>")]
pub fn list_order_handler(order_id: i32) -> Result<String, NotFound<String>> {
    let order = read::list_order(order_id)?;
    let response = Response { body: ResponseBody::Order(order) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/new_order", format = "application/json", data = "<order>")]
pub fn create_order_handler(order: Json<NewOrder>) -> Created<String> {
    create::create_order(order)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = list_orders_handler();
        // assert!(Ok(result));
    }
}