#[macro_use] extern crate rocket;
use api::user::user_handler;
use api::product::product_handler;
use api::order::order_handler;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/store", routes![
            // Users
            user_handler::list_users_handler, 
            user_handler::list_user_handler,
            user_handler::create_user_handler,
            user_handler::update_user_handler,
            user_handler::delete_user_handler,
            // Orders
            order_handler::list_orders_handler, 
            order_handler::list_order_handler,
            order_handler::create_order_handler,
            order_handler::update_order_handler,
            order_handler::delete_order_handler,
            // Products
            product_handler::list_products_handler, 
            product_handler::list_product_handler,
            product_handler::create_product_handler,
            product_handler::update_product_handler,
            product_handler::delete_product_handler,
        ])
}