use diesel::prelude::*;
use domain::models::{NewProduct, Product};
use infrastructure::establish_connection;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use shared::response_models::{Response, ResponseBody};

pub fn create_product(product: Json<NewProduct>) -> Created<String> {
    use infrastructure::schema::products;

    let product = product.into_inner();

    match diesel::insert_into(products::table)
        .values(&product)
        .get_result::<Product>(&mut establish_connection())
    {
        Ok(product) => {
            let response = Response {
                body: ResponseBody::Product(product),
            };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        }
        Err(err) => {
            panic!("Database error - {}", err);
        }
    }
}
