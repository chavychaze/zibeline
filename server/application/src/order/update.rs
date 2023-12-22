use diesel::prelude::*;
use domain::models::Order;
use infrastructure::establish_connection;
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};

pub fn update_order(order_id: i32) -> Result<Order, NotFound<String>> {
    use infrastructure::schema::orders::dsl::*;

    match diesel::update(orders.find(order_id))
        .set(is_delivered.eq(true))
        .get_result::<Order>(&mut establish_connection())
    {
        Ok(order) => Ok(order),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response {
                    body: ResponseBody::Message(format!(
                        "Error registering order with id {} - {}",
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
