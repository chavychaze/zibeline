use shared::response_models::{Response, ResponseBody};
use application::product::{create, read, update, delete};
use domain::models::{Product, NewProduct};
use rocket::{get, post,patch, delete};
use rocket::response::status::{NotFound, Created};
use rocket::serde::json::Json;

#[get("/products")]
pub fn list_products_handler() -> String {
    let products: Vec<Product> = read::list_products();
    let response = Response { body: ResponseBody::Products(products) };

    serde_json::to_string(&response).unwrap()
}

#[get("/product/<product_id>")]
pub fn list_product_handler(product_id: i32) -> Result<String, NotFound<String>> {
    let product = read::list_product(product_id)?;
    let response = Response { body: ResponseBody::Product(product) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[patch("/update/<product_id>")]
pub fn update_product_handler(product_id: i32) -> Result<String, NotFound<String>> {
    let product = update::update_product(product_id)?; 
    let response = Response { body: ResponseBody::Product(product) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[delete("/delete/<product_id>")]
pub fn delete_product_handler(product_id: i32) -> Result<String, NotFound<String>> {
    let products = delete::delete_product(product_id)?;
    let response = Response { body: ResponseBody::Products(products) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/product", format = "application/json", data = "<product>")]
pub fn create_product_handler(product: Json<NewProduct>) -> Created<String> {
    create::create_product(product)
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