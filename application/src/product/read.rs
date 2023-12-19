use domain::models::Product;
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::NotFound;

pub fn list_product(product_id: i32) -> Result<Product, NotFound<String>> {
    use infrastructure::schema::products;

    match products::table.find(product_id).first::<Product>(&mut establish_connection()) {
        Ok(product) => Ok(product),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error selecting product with id {} - {}", product_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    }
}

pub fn list_products() -> Vec<Product> {
    use infrastructure::schema::products;

    match products::table.select(products::all_columns).load::<Product>(&mut establish_connection()) {
        Ok(mut products) => {
            products.sort();
            products
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}