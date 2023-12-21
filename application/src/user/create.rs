use diesel::prelude::*;
use domain::models::{NewUser, User};
use infrastructure::establish_connection;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use shared::response_models::{Response, ResponseBody};

pub fn create_user(user: Json<NewUser>) -> Created<String> {
    use infrastructure::schema::users;

    let user = user.into_inner();

    match diesel::insert_into(users::table)
        .values(&user)
        .get_result::<User>(&mut establish_connection())
    {
        Ok(user) => {
            let response = Response {
                body: ResponseBody::User(user),
            };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        }
        Err(err) => {
            panic!("Database error - {}", err);
        }
    }
}
