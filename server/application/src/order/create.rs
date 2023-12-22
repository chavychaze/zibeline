use diesel::prelude::*;
use domain::models::{NewOrder, Order};
use infrastructure::establish_connection;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use shared::response_models::{Response, ResponseBody};

pub fn create_order(order: Json<NewOrder>) -> Created<String> {
    use infrastructure::schema::orders;

    let order = order.into_inner();

    match diesel::insert_into(orders::table)
        .values(&order)
        .get_result::<Order>(&mut establish_connection())
    {
        Ok(order) => {
            let response = Response {
                body: ResponseBody::Order(order),
            };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        }
        Err(err) => {
            panic!("Database error - {}", err);
        }
    }
}
