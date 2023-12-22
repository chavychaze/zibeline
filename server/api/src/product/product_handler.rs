use application::product::{create, delete, read, update};
use domain::models::{NewProduct, Product};
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket::{delete, get, patch, post};
use shared::response_models::{Response, ResponseBody};

#[get("/products")]
pub fn list_products_handler() -> String {
    let products: Vec<Product> = read::list_products();
    let response = Response {
        body: ResponseBody::Products(products),
    };

    serde_json::to_string(&response).unwrap()
}

#[get("/product/<product_id>")]
pub fn list_product_handler(product_id: i32) -> Result<String, NotFound<String>> {
    let product = read::list_product(product_id)?;
    let response = Response {
        body: ResponseBody::Product(product),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/product", format = "application/json", data = "<product>")]
pub fn create_product_handler(product: Json<NewProduct>) -> Created<String> {
    create::create_product(product)
}

#[patch("/product/<product_id>")]
pub fn update_product_handler(product_id: i32) -> Result<String, NotFound<String>> {
    let product = update::update_product(product_id)?;
    let response = Response {
        body: ResponseBody::Product(product),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[delete("/product/<product_id>")]
pub fn delete_product_handler(product_id: i32) -> Result<String, NotFound<String>> {
    let products = delete::delete_product(product_id)?;
    let response = Response {
        body: ResponseBody::Products(products),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = list_products_handler();
        // assert!(Ok(result));
    }
}
