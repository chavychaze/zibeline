use diesel::prelude::*;
use domain::models::Order;
use infrastructure::establish_connection;
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};

pub fn list_order(order_id: i32) -> Result<Order, NotFound<String>> {
    use infrastructure::schema::orders;

    match orders::table
        .find(order_id)
        .first::<Order>(&mut establish_connection())
    {
        Ok(order) => Ok(order),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response {
                    body: ResponseBody::Message(format!(
                        "Error selecting order with id {} - {}",
                        order_id, err
                    )),
                };
                Err(NotFound(serde_json::to_string(&response).unwrap()))
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn list_orders() -> Vec<Order> {
    use infrastructure::schema::orders;

    match orders::table
        .select(orders::all_columns)
        .load::<Order>(&mut establish_connection())
    {
        Ok(mut orders) => {
            orders.sort();
            orders
        }
        Err(err) => {
            panic!("Database error - {}", err);
        }
    }
}
