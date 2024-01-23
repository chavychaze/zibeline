use diesel::prelude::*;
use domain::models::Product;
use infrastructure::establish_connection;
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};

pub fn update_product(product_id: i32) -> Result<Product, NotFound<String>> {
    use infrastructure::schema::product::dsl::*;

    match diesel::update(product.find(product_id))
        .set(is_delete.eq(false))
        .get_result::<Product>(&mut establish_connection())
    {
        Ok(prod) => Ok(prod),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response {
                    body: ResponseBody::Message(format!(
                        "Error registering product with id {} - {}",
                        product_id, err
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
