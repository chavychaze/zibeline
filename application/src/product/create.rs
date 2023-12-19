use domain::models::{Product, NewProduct};
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::Created;
use rocket::serde::json::Json;

pub fn create_product(product: Json<NewProduct>) -> Created<String> {
    use infrastructure::schema::products;

    let product = product.into_inner();

    match diesel::insert_into(products::table).values(&product).get_result::<Product>(&mut establish_connection()) {
        Ok(product) => {
            let response = Response { body: ResponseBody::Product(product) };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}