use application::order::{create, delete, read, update};
use domain::models::{NewOrder, Order};
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket::{delete, get, patch, post};
use shared::response_models::{Response, ResponseBody};

#[get("/orders")]
pub fn list_orders_handler() -> String {
    let orders: Vec<Order> = read::list_orders();
    let response = Response {
        body: ResponseBody::Orders(orders),
    };

    serde_json::to_string(&response).unwrap()
}

#[get("/order/<order_id>")]
pub fn list_order_handler(order_id: i32) -> Result<String, NotFound<String>> {
    let order = read::list_order(order_id)?;
    let response = Response {
        body: ResponseBody::Order(order),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[patch("/order/<order_id>")]
pub fn update_order_handler(order_id: i32) -> Result<String, NotFound<String>> {
    let order = update::update_order(order_id)?;
    let response = Response {
        body: ResponseBody::Order(order),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[delete("/order/<order_id>")]
pub fn delete_order_handler(order_id: i32) -> Result<String, NotFound<String>> {
    let orders = delete::delete_order(order_id)?;
    let response = Response {
        body: ResponseBody::Orders(orders),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/order", format = "application/json", data = "<order>")]
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
