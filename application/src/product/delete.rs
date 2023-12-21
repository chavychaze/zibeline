use diesel::prelude::*;
use domain::models::Product;
use infrastructure::establish_connection;
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};

pub fn delete_product(product_id: i32) -> Result<Vec<Product>, NotFound<String>> {
    use infrastructure::schema::products;
    use infrastructure::schema::products::dsl::*;

    let response: Response;

    let num_deleted = match diesel::delete(products.filter(id.eq(product_id)))
        .execute(&mut establish_connection())
    {
        Ok(count) => count,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response {
                    body: ResponseBody::Message(format!(
                        "Error deleting product with id {} - {}",
                        product_id, err
                    )),
                };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    };

    if num_deleted > 0 {
        match products::table
            .select(products::all_columns)
            .load::<Product>(&mut establish_connection())
        {
            Ok(mut products_) => {
                products_.sort();
                Ok(products_)
            }
            Err(err) => {
                panic!("Database error - {}", err);
            }
        }
    } else {
        response = Response {
            body: ResponseBody::Message(format!("Error - no product with id {}", product_id)),
        };
        Err(NotFound(serde_json::to_string(&response).unwrap()))
    }
}
